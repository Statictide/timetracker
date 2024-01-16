use askama_axum::Template;

#[derive(Template)]
#[template(path = "pages/hello.html")]
pub struct HelloTemplate;

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "pages/another-page.html")]
pub struct AnotherPageTemplate;

#[derive(Template)]
#[template(path = "components/todo-list.html")]
pub struct TodoList {
    pub todos: Vec<String>,
}
