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

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)
        .expect("There should only be one global subscriber");

    let app_state = SyncAppState::default();

    info!("initializing router...");
    let assets_path = std::env::current_dir().unwrap();
    let assets_path = assets_path.to_str().unwrap();

    let api_router = Router::new()
        .route("/todos", post(routes::add_todo))
        .with_state(app_state);

    
    let pages_router = Router::new()
        .route("/", get(routes::index))
        .route("/another-page", get(routes::another_page));

    let app = Router::new()
        .nest("/", pages_router)
        .nest("/api", api_router)
        .nest_service("/assets", ServeDir::new(format!("{assets_path}/assets")));

    let ip = "127.0.0.1";
    let port = 3000;
    let listener = tokio::net::TcpListener::bind(format!("{ip}:{port}"))
        .await
        .unwrap();

    let server = axum::serve(listener, app);
    info!("Server running at http://{ip}:{port}");
    server.await.unwrap();
}
