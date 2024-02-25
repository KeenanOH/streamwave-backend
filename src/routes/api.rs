use axum::extract::{Query, State};
use axum::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::{EntityTrait};
use sea_orm::sea_query::OnConflict;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::models::prelude::Shows;
use crate::models::sea_orm_active_enums::TranslationType;
use crate::models::shows;
use crate::provider::scraper;

#[derive(Deserialize)]
pub struct SearchQuery {
    #[serde(rename = "q")]
    query: String
}

#[derive(Deserialize, Serialize)]
pub struct ShowResult {
    show_id: String,
    name: String,
    translation_type: TranslationType,
    episodes: u32,
    image_url: String
}

pub async fn search(query: Query<SearchQuery>, State(state): State<AppState>) -> Json<Vec<ShowResult>> {
    let search_results = scraper::search(
        state.client,
        "https://api.allanime.day",
        query.query.as_str()
    ).await;
    let mut results: Vec<ShowResult> = Vec::new();

    for show in search_results.data.shows.edges {
        if show.english_name.is_some() && show.available_episodes.dub > 0 {
            results.push(ShowResult {
                show_id: show.id.as_str().to_string(),
                name: show.english_name.unwrap(),
                translation_type: TranslationType::Dub,
                episodes: show.available_episodes.dub,
                image_url: show.image_url.as_str().to_string()
            })
        }

        if show.available_episodes.sub > 0 {
            results.push(ShowResult {
                show_id: show.id,
                name: show.name,
                translation_type: TranslationType::Sub,
                episodes: show.available_episodes.sub,
                image_url: show.image_url
            })
        }
    }

    let show_models: Vec<shows::ActiveModel> = results.iter()
        .map(|show_result| {
            shows::ActiveModel {
                show_id: Set(show_result.show_id.to_owned()),
                translation_type: Set(show_result.translation_type.to_owned()),
                name: Set(show_result.name.to_owned()),
                image_url: Set(show_result.image_url.to_owned()),
                ..Default::default()
            }
        })
        .collect();

    tokio::spawn(async move {
        let _ = Shows::insert_many(show_models)
            .on_conflict(
                OnConflict::columns([shows::Column::ShowId, shows::Column::TranslationType])
                    .update_column(shows::Column::ShowId)
                    .to_owned()
            )
            .exec(&state.db)
            .await;
    });

    return Json(results);
}


#[derive(Deserialize)]
pub struct EpisodeQuery {
    show_id: String,
    translation_type: String,
    episode_num: String
}

#[derive(Deserialize, Serialize)]
pub struct EpisodeResult {
    streaming_url: Option<String>,
    source_name: Option<String>
}

pub async fn episode(query: Query<EpisodeQuery>, State(state): State<AppState>) -> Json<EpisodeResult> {
    let episode_response = scraper::get_episode(
        state.client.clone(),
        "https://api.allanime.day",
        query.show_id.as_str(),
        query.translation_type.as_str(),
        query.episode_num.as_str()
    ).await;

    let [streaming_url, source_name] = scraper::get_best_streaming_link(state.client, episode_response).await;

    return Json(EpisodeResult {
        streaming_url, source_name
    });
}
