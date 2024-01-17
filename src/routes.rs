use crate::{templates::*, SyncAppState};
use axum::{extract::State, response::IntoResponse, Form};
use serde::Deserialize;

pub async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

pub async fn time_page() -> impl IntoResponse {
    TimePageTemplate {}
}

#[derive(Deserialize)]
pub struct AddTodoRequest {
    pub todo: String,
}

pub async fn add_todo(
    State(state): State<SyncAppState>,
    Form(todo): Form<AddTodoRequest>,
) -> impl IntoResponse {
    let mut lock = state.todos.lock().unwrap();
    lock.push(todo.todo);

    TodoList {
        todos: lock.clone(),
    }
}
