mod routes;
mod templates;
use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
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
    use routes::*;
    let app_state = SyncAppState::default();
    info!("initializing router...");
    let pages_router = Router::new()
        .route("/", get(index))
        .route("/todo", get(todo_page))
        .route("/time", get(time_page))
        .route("/hello", get(hello));

    let htmx_router = Router::new()
        .route("/todos", post(routes::add_todo))
        .route("/hello", get(|| async { "Hello!" }))
        .with_state(app_state);

    let app: Router = Router::new()
        .nest("/", pages_router)
        .nest("/hx", htmx_router)
        .route("/api", get(|| async { "Reserved for the API" }))
        .nest_service("/assets", ServeDir::new("assets")) // serves static files
        .layer(tower_livereload::LiveReloadLayer::new()) // reloads frontend on server restart
        ;

    info!("starting server...");
    return Ok(app.into());
}
