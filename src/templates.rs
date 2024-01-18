use askama_axum::Template;

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "pages/time.html")]
pub struct TimePageTemplate {
    hours: Vec<&'static str>,
    minutes: Vec<&'static str>,
    days: Vec<&'static str>,
}

impl TimePageTemplate {
    pub fn new() -> Self {
        let hours = 
            vec!["00", "01", "02", "03", "04", "05",
                "06", "07", "08", "09", "10", "11",
                "12", "13", "14", "15", "16", "17",
                "18", "19", "20", "21", "22", "23"];

        let minutes = 
            vec!["00", "15", "30", "45"];

        let days = 
            vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

        return Self {
            hours,
            minutes,
            days,
        }
    }
    
}

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
