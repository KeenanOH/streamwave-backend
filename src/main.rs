mod provider;
mod routes;
mod models;
mod auth;

use axum::{routing::get, Router, Extension, middleware};
use axum::routing::post;
use reqwest::Client as ReqwestClient;
use sea_orm::{Database, DatabaseConnection};
use shuttle_secrets::SecretStore;
use crate::auth::oauth_client::build_oauth_client;

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
    client: ReqwestClient,
}

#[derive(Clone)]
pub struct OAuthInfo {
    oauth_id: String,
    oauth_redirect_uri: String,
    oauth_success_url: String
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let db_url = secrets.get("DB_URL").unwrap();
    let oauth_id = secrets.get("GOOGLE_OAUTH_CLIENT_ID").unwrap();
    let oauth_secret = secrets.get("GOOGLE_OAUTH_CLIENT_SECRET").unwrap();
    let oauth_redirect_uri = secrets.get("GOOGLE_OAUTH_REDIRECT_URI").unwrap();
    let oauth_success_url = secrets.get("OAUTH_SUCCESS_URL").unwrap();

    let db = Database::connect(db_url).await.unwrap();
    let client = ReqwestClient::new();

    let state = AppState {
        db,
        client,
    };

    let oauth_client = build_oauth_client(oauth_id.clone(), oauth_secret);

    let auth_router = Router::new()
        .route("/google_callback", get(auth::callback::google_callback))
        .route("/signin", get(auth::login::login))
        .route_layer(Extension(OAuthInfo { oauth_id, oauth_redirect_uri, oauth_success_url }));

    let list_router = Router::new()
        .route("/", get(routes::list::get_list))
        .route("/add", post(routes::list::add_show_to_list))
        .route("/delete", post(routes::list::remove_show_from_list))
        .layer(middleware::from_fn_with_state(state.clone(), auth::middleware::check_authorized));

    let api_router = Router::new()
        .route("/search", get(routes::api::search))
        .route("/episode", get(routes::api::episode))
        .route("/show", get(routes::shows::get_show));

    let router = Router::new()
        .nest("/api/auth", auth_router)
        .nest("/api", api_router)
        .nest("/list", list_router)
        .route("/", get(root))
        .layer(Extension(oauth_client))
        .with_state(state);


    Ok(router.into())
}
