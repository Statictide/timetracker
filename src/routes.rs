use crate::{templates::*, SyncAppState};
use axum::{extract::State, response::IntoResponse, Form};
use serde::Deserialize;
use tracing::info;

pub async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

pub async fn time_page() -> impl IntoResponse {
    TimePageTemplate {}
}

pub async fn todo_page() -> impl IntoResponse {
    TodoPageTemplate {}
}

pub async fn hello() -> impl IntoResponse {
    HelloPageTemplate {}
}

pub async fn get_todo(
    State(state): State<SyncAppState>
) -> impl IntoResponse {
    let lock = state.todos.lock().unwrap();

    TodoList {
        todos: lock.clone(),
    }
}


#[derive(Deserialize)]
pub struct AddTodoRequest {
    pub todo: String,
}

pub async fn add_todo(
    State(state): State<SyncAppState>,
    Form(todo): Form<AddTodoRequest>,
) -> impl IntoResponse {
    info!("adding todo: {}", todo.todo);

    let mut lock = state.todos.lock().unwrap();
    lock.push(todo.todo);

    TodoList {
        todos: lock.clone(),
    }
}
