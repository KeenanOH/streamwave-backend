use axum::Extension;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::{CookieJar};
use chrono::{Local, TimeDelta};
use oauth2::basic::BasicClient;
use oauth2::{AuthorizationCode, TokenResponse};
use oauth2::reqwest::async_http_client;
use sea_orm::{EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::OnConflict;
use crate::{AppState, OAuthInfo};
use crate::models::{sessions, users};
use crate::models::prelude::Users;
use sea_orm::ColumnTrait;
use crate::auth::models::{AuthRequest, UserProfile};

pub async fn google_callback(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<AuthRequest>,
    Extension(oauth_client): Extension<BasicClient>,
    Extension(oauth_info): Extension<OAuthInfo>
) -> Result<impl IntoResponse, impl IntoResponse> {

    /* Get OAUTH token */
    let oauth_token_response = oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await;

    let token = match oauth_token_response {
        Ok(response ) => response,
        Err(error) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get oauth token {error}")))
        }
    };

    /* Retrieve profile from Google */
    let profile_response = state.client.get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(token.access_token().secret().to_owned())
        .send()
        .await;

    let profile = match profile_response {
        Ok(profile) => profile,
        Err(error) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to retrieve profile {error}")))
        }
    };

    /* Deserialize profile from Google */
    let profile = profile.json::<UserProfile>().await;
    let profile = match profile {
        Ok(profile) => profile,
        Err(error) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse profile: {error}")))
        }
    };

    /* Build the session's cookie */
    let cookie = Cookie::build(("sid", token.access_token().secret().to_owned()))
        .path("/")
        .same_site(SameSite::Lax)
        .secure(true)
        .http_only(true)
        .build();


    /* Create user. if already exists, ignore*/
    let user = users::ActiveModel {
        email: Set(profile.email.clone()),
        ..Default::default()
    };

    let create_user_response = Users::insert(user)
        .on_conflict(
            OnConflict::column(users::Column::Email)
                .update_column(users::Column::Email)
                .to_owned()
        )
        .exec(&state.db)
        .await;

    match create_user_response {
        Ok(_) => {}
        Err(error) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error creating user: {error}")))
        }
    };

    /* Fetch the user */
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(profile.email))
        .one(&state.db)
        .await;
    let user = match user {
        Ok(user) => user,
        Err(error) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error finding user: {error}")))
        }
    };
    let user = match user {
        Some(user) => user,
        None => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "This user doesn't exist!".to_string()))
        }
    };

    /* Create the session in the database */
    let secs: i64 = token.expires_in().unwrap().as_secs().try_into().unwrap();
    let expires_at = Local::now().naive_local() + TimeDelta::seconds(secs);
    let session = sessions::ActiveModel {
        user_id: Set(user.id),
        session_id: Set(token.access_token().secret().to_owned()),
        expires_at: Set(expires_at.and_utc()),
        ..Default::default()
    };

    if let Err(e) = sessions::Entity::insert(session)
        .on_conflict(
            OnConflict::column(sessions::Column::UserId)
                .update_column(sessions::Column::SessionId)
                .to_owned()
        )
        .exec(&state.db)
        .await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error creating session: {e}")))
    }

    /* Return the response */
    Ok((
        jar.add(cookie),
        Redirect::to(&oauth_info.oauth_success_url)
    ))
}
