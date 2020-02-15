// #![feature(decl_macro)]

use rocket_contrib::json::Json;

use crate::models::Todo;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world"
}

#[get("/todos")]
pub fn todos() -> Json<Vec<Todo>> {
    Json(vec![Todo {
        id: 1,
        title: "Read rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    }])
}

#[post("/todo", data = "<todo>")]
pub fn todo_post(todo: Json<Todo>) -> String {
    format!("Accepted post request: {:?}", todo.0)
}

#[get("/todo/<id>")]
pub fn todo(id: u32) -> String {
    let todo = Todo {
        id,
        title: "Read rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    };

    format!("{:?}", todo)
}
