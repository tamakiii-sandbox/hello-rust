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

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello, pages, files])
        .launch();
}
