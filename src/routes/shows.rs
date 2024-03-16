use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{Condition, EntityTrait, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::models::shows;

#[derive(Deserialize, Serialize)]
pub struct GetShowQuery {
    show_id: String,
    translation_type: String
}

pub async fn get_show(
    State(state): State<AppState>,
    Query(query): Query<GetShowQuery>
) -> Result<Json<shows::Model>, (StatusCode, String)> {
    let show = match shows::Entity::find()
        .filter(
            Condition::all()
                .add(shows::Column::ShowId.eq(query.show_id))
                .add(shows::Column::TranslationType.eq(query.translation_type))
        )
        .one(&state.db)
        .await {
        Ok(show) => show,
        Err(error) =>
            return Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))
    };

    return match show {
        Some(show) => Ok(Json(show)),
        None => Err((StatusCode::INTERNAL_SERVER_ERROR, "show not found".to_string()))
    }
}
