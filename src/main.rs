mod templates;

use std::sync::{Arc, Mutex};

use axum::{response::IntoResponse, routing::{get, post}, Router, extract::State, Form};
use tracing::info;
use tower_http::services::ServeDir;
use serde::Deserialize;

struct AppState {
    todos: Mutex<Vec<String>>,
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)
        .expect("There should only be one global subscriber");

    let app_state = Arc::new(AppState {
        todos: Mutex::new(vec![]),
    });

    info!("initializing router...");

    let assets_path = std::env::current_dir().unwrap();
    let assets_path = assets_path.to_str().unwrap();
    
    let api_router = Router::new()
        .route("/hello", get(hello_from_the_server))
        .route("/todos", post(add_todo))
        .with_state(app_state);

    let app = Router::new()
        .nest("/api", api_router)
        .route("/", get(index))
        .route("/another-page", get(another_page))
        .nest_service("/assets", ServeDir::new(format!("{assets_path}/assets")));

    let port = 3000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await.unwrap();

    let server = axum::serve(listener, app);
    info!("Server running at http://127.0.0.1:{port}");
    server.await.unwrap();
}

async fn index() -> impl IntoResponse {
    use templates::*;

    info!("Request");
    let template = HelloTemplate {};
    HtmlTemplate(template)
}

async fn another_page() -> impl IntoResponse {
    use templates::*;

    let template = AnotherPageTemplate {};
    HtmlTemplate(template)
}


async fn hello_from_the_server() -> &'static str {
    "Hello!"
}

#[derive(Deserialize)]
struct TodoRequest {
    pub todo: String
}

async fn add_todo(
    State(state): State<Arc<AppState>>,
    Form(todo): Form<TodoRequest>,
) -> impl IntoResponse {
    use templates::*;
    
    let mut lock = state.todos.lock().unwrap();
    lock.push(todo.todo);

    let template = TodoList {
        todos: lock.clone(),
    };

    HtmlTemplate(template)
}