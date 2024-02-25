use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use sea_orm::{EntityTrait, ModelTrait};
use crate::AppState;
use crate::models::{sessions, users};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use crate::auth::models::UserExtension;

pub async fn check_authorized(
    State(state): State<AppState>,
    jar: CookieJar,
    mut req: Request<axum::body::Body>,
    next: Next
) -> Result<impl IntoResponse, impl IntoResponse> {
    let Some(cookie) = jar.get("sid").map(|cookie| cookie.value().to_owned()) else {
        return Err((StatusCode::UNAUTHORIZED, "No cookie found!".to_string()));
    };

    let session = match sessions::Entity::find()
        .filter(sessions::Column::SessionId.eq(cookie))
        .one(&state.db)
        .await {
        Ok(session) => session,
        Err(_) => {
            return Err((StatusCode::UNAUTHORIZED, "Failed to get session!".to_string()))
        }
    };
    let Some(session) = session else {
        return Err((StatusCode::UNAUTHORIZED, "Failed to get session".to_string()))
    };

    let user = match session
        .find_related(users::Entity)
        .one(&state.db)
        .await {
        Ok(user) => user,
        Err(_) => {
            return Err((StatusCode::UNAUTHORIZED, "Failed to get user".to_string()))
        }
    };
    let Some(user) = user else {
        return Err((StatusCode::UNAUTHORIZED, "Failed to get user".to_string()))
    };

    req.extensions_mut().insert(UserExtension {
        id: user.id,
        email: user.email
    });

    Ok(next.run(req).await)
}
