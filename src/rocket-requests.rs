#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;

#[get("/hello/<name>")]
fn index(name: &RawStr) -> String {
    format!("Hello, {}", name.as_str())
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} years old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello])
        .launch();
}
