use axum::{extract::State, response::IntoResponse, Form};
use serde::Deserialize;
use tracing::info;

use crate::{templates::*, SyncAppState};

pub async fn index() -> impl IntoResponse {
    info!("Request");
    let template = HelloTemplate {};
    HtmlTemplate(template)
}

pub async fn another_page() -> impl IntoResponse {
    let template = AnotherPageTemplate {};
    HtmlTemplate(template)
}

pub async fn hello_from_the_server() -> &'static str {
    "Hello!"
}

#[derive(Deserialize)]
pub struct TodoRequest {
    pub todo: String,
}

pub async fn add_todo(
    State(state): State<SyncAppState>,
    Form(todo): Form<TodoRequest>,
) -> impl IntoResponse {
    use crate::templates::*;

    let mut lock = state.todos.lock().unwrap();
    lock.push(todo.todo);

    let template = TodoList {
        todos: lock.clone(),
    };

    HtmlTemplate(template)
}
