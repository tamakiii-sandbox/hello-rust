// #![feature(decl_macro)]

use rocket::response::status;
use rocket_contrib::json::Json;

use crate::models::Todo;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world"
}

#[get("/todos")]
pub fn todos() -> status::Accepted<Json<Vec<Todo>>> {
    status::Accepted(Some(Json(vec![sample_entity(1)])))
}

#[post("/todo", data = "<todo>")]
pub fn todo_post(todo: Json<Todo>) -> status::Created<Json<Todo>> {
    status::Created(format!("Accepted post request: {:?}", todo.0), Some(todo))
}

#[get("/todo/<id>")]
pub fn todo(id: u32) -> Json<Todo> {
    let todo = sample_entity(id);
    Json(todo)
}

fn sample_entity(id: u32) -> Todo {
    Todo {
        id: id,
        title: "Read rocket tutorial".into(),
        description: "Read https://rocket.rs/guide/quickstart/".into(),
        done: false,
    }
}
