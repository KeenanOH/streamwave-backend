use axum::extract::{State};
use axum::{Extension, Json};
use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::auth::models::UserExtension;
use crate::models::{shows, users_shows};

pub async fn get_list(
    State(state): State<AppState>,
    Extension(user): Extension<UserExtension>
) -> Result<Json<Vec<shows::Model>>, (StatusCode, String)> {
    let user_shows = match users_shows::Entity::find()
        .filter(users_shows::Column::UserId.eq(user.id))
        .find_with_related(shows::Entity)
        .all(&state.db)
        .await {
        Ok(user_shows) => user_shows,
        Err(error) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))
        }
    };

    let list: Vec<shows::Model> = user_shows.iter()
        .map(|(_user_show, show)| {
            let show = show.first().unwrap();
            return show.clone()
        })
        .collect();

    Ok(Json(list))
}

#[derive(Deserialize, Serialize)]
pub struct ShowData {
    pub show_id: String,
    pub translation_type: String
}

pub async fn add_show_to_list(
    State(state): State<AppState>,
    Extension(user): Extension<UserExtension>,
    Json(data): Json<ShowData>,
) -> Result<Json<shows::Model>, (StatusCode, String)> {
    let show = match shows::Entity::find()
        .filter(
            Condition::all()
                .add(shows::Column::ShowId.eq(data.show_id))
                .add(shows::Column::TranslationType.eq(data.translation_type))
        )
        .one(&state.db)
        .await {
        Ok(show) => show,
        Err(error) =>
            return Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string())),
    };
    let Some(show) = show else {
        return Err((StatusCode::BAD_REQUEST, "this show is not in the database".to_string()))
    };

    let user_show = users_shows::ActiveModel {
        show_id: Set(show.id),
        user_id: Set(user.id),
        ..Default::default()
    };

    let _ = user_show.insert(&state.db).await;

    Ok(Json(show))
}

pub async fn remove_show_from_list(
    State(state): State<AppState>,
    Extension(user): Extension<UserExtension>,
    Json(data): Json<ShowData>
) -> Result<StatusCode, (StatusCode, String)> {
    let show = match shows::Entity::find()
        .filter(
            Condition::all()
                .add(shows::Column::ShowId.eq(data.show_id))
                .add(shows::Column::TranslationType.eq(data.translation_type))
        )
        .one(&state.db)
        .await {
        Ok(show) => show,
        Err(error) =>
            return Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string())),
    };
    let Some(show) = show else {
        return Err((StatusCode::BAD_REQUEST, "this show is not in the database".to_string()))
    };

    let result = users_shows::Entity::delete_many()
        .filter(
            Condition::all()
                .add(users_shows::Column::ShowId.eq(show.id))
                .add(users_shows::Column::UserId.eq(user.id))
        )
        .exec(&state.db)
        .await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))
    }
}
