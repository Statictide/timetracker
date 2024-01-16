use axum::{extract::State, response::IntoResponse, Form};
use serde::Deserialize;
use crate::{templates::*, SyncAppState};

pub async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

pub async fn another_page() -> impl IntoResponse {
    AnotherPageTemplate {}
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
