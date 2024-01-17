use askama_axum::Template;

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "pages/time.html")]
pub struct TimePageTemplate;

#[derive(Template)]
#[template(path = "pages/hello.html")]
pub struct HelloPageTemplate;

#[derive(Template)]
#[template(path = "pages/todo.html")]
pub struct TodoPageTemplate;

#[derive(Template)]
#[template(path = "components/todo-list.html")]
pub struct TodoList {
    pub todos: Vec<String>,
}
