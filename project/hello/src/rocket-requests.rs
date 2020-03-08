#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

const DIR_ROOT: &'static str = "/workspaces/hello-rust";

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

#[get("/pages/<path..>")]
fn pages(path: PathBuf) -> String {
    format!("page: {:?}", path)
}

#[get("/files/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(DIR_ROOT).join(file)).ok()
}

mod cookies {
    use rocket::http::{Cookie, Cookies};

    #[get("/")]
    pub fn index(cookies: Cookies) -> String {
        let results = cookies.get("count")
            .map(|value| format!("Message: {}", value));

        match results {
            Some(r) => r,
            None => format!("Nothing found"),
        }
    }

    #[get("/++")]
    pub fn increment(mut cookies: Cookies) -> String {
        let count: i32 = match cookies.get("count") {
            Some(c) => c.value().parse::<i32>().unwrap() + 1,
            None => 0,
        };

        cookies.add(Cookie::new("count", count.to_string()));

        format!("count: {}", count)
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello, pages, files])
        .mount("/cookies", routes![cookies::index, cookies::increment])
        .launch();
}
