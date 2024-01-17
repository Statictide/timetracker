mod routes;
mod templates;
use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

type SyncAppState = Arc<AppState>;

struct AppState {
    todos: Mutex<Vec<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            todos: Default::default(),
        }
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app_state = SyncAppState::default();

    info!("initializing router...");
    let pages_router = Router::new()
        .route("/", get(routes::index))
        .route("/time", get(routes::time_page));

    let api_router = Router::new()
        .route("/todos", post(routes::add_todo))
        .with_state(app_state);

    let app: Router = Router::new()
        .nest("/", pages_router)
        .nest("/api", api_router)
        .nest_service("/assets", ServeDir::new("assets")) // serves static files
        .layer(TraceLayer::new_for_http()) // log http requests
        .layer(tower_livereload::LiveReloadLayer::new()) // reloads frontend on server restart
        ;

    return Ok(app.into());
}
