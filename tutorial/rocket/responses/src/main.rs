#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> String {
    format!("Hello")
}

mod id {
    use rocket::response::status;
    use rocket::response::content;
    use rocket::http::Status;

    #[post("/<id>")]
    pub fn post(id: usize) -> status::Accepted<String> {
        status::Accepted(Some(format!("id: {}", id)))
    }

    #[get("/json")]
    pub fn json() -> content::Json<&'static str> {
        content::Json("{\"message\": \"hello\"}")
    }

    #[get("/fail")]
    pub fn fail() -> Status {
        Status::NotAcceptable
    }
}

mod responder {
    use rocket::request::Request;
    use rocket::response::{self, Response, Responder};
    use rocket::http::ContentType;
    use std::io::Cursor;

    // #[derive(Responder)]
    // #[response(status=500, content_type="json")]
    // struct MyResponder {
    //     inner: OtherResponder,
    //     header: SomeResponder,
    //     more: YetAnotherHeader,
    //     #[response(ignore)]
    //     unrelated: MyType,
    // }

    pub struct Person {
        name: String,
        age: u16,
    }

    impl<'r> Responder<'r> for Person {
        // fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        fn respond_to(self, _: &Request) -> response::Result<'r> {
            Response::build()
                .header(ContentType::Plain)
                .sized_body(Cursor::new(format!("{}:{}", self.name, self.age)))
                .raw_header("X-Person-Name", self.name)
                .raw_header("X-Person-Age", self.age.to_string())
                .header(ContentType::new("application", "x-person"))
                .ok()
        }
    }

    #[get("/")]
    pub fn index() -> Person {
        Person {
            name: "neko".into(),
            age: 6,
        }
    }
}

mod task {
    use rocket_contrib::json::Json;
    // use serde::ser::{Serialize, SerializeStruct, Serializer};
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Task {
        id: u32,
        name: String,
    }

    #[get("/<id>")]
    pub fn id(id: u32) -> Json<Task> {
        Json(Task {
            id,
            name: "hello".into(),
        })
    }
}

mod template {
    use rocket_contrib::templates::Template;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Task {
        id: u32,
        name: String,
    }

    #[derive(Serialize)]
    struct MyData {
        tasks: Vec<Task>,
    }

    #[get("/")]
    pub fn index() -> Template {
        let context = MyData {
            tasks: vec![
                Task { id: 1, name: "hoge".to_string()},
                Task { id: 2, name: "fuga".to_string()},
            ]
        };

        // When your application is compiled in debug mode (without the --release flag passed to cargo), templates are automatically reloaded when they are modified on supported platforms.
        // This means that you don't need to rebuild your application to observe template changes: simply refresh! In release builds, reloading is disabled.
        Template::render("index", &context)
    }
}

mod uri {
    #[get("/person/<name>?<age>")]
    pub fn person(name: String, age: Option<u8>) -> String {
        format!("Hello, {} ({})", name, (match age {
            Some(a) => a.to_string(),
            None => "?".to_string(),
        }))
    }

    #[get("/list")]
    pub fn list() -> String {
        let list = [
            format!("{}", uri!(person: "Mike Smith", 28)),
            format!("{}", uri!(person:        "Mike", 28)),
            format!("{}", uri!(person: name = "Mike", age = 28)),
            format!("{}", uri!(person: age = 28, name = "Mike")),
            format!("{}", uri!("/api", person: name = "Mike", age = 28)),
            format!("{}", uri!(person: "Mike", _)),
            format!("{}", uri!(person: name = "Mike", age = _)),
        ];

        list.join("\n")
    }

    use rocket::http::RawStr;
    use rocket::request::Form;

    #[derive(FromForm, UriDisplayQuery)]
    pub struct UserDetails<'r> {
        age: Option<usize>,
        nickname: &'r RawStr,
    }

    #[post("/user/<id>?<details..>")]
    pub fn add_user(id: usize, details: Form<UserDetails>) -> String {
        format!("{} added (age={}, id={})", details.nickname, (details.age).unwrap().to_string(), id)
    }
}

mod typed_uri {
    use std::fmt;
    use rocket::http::RawStr;
    use rocket::http::uri::{Formatter, UriDisplay, FromUriParam, Query};
    use rocket::request::Form;

    #[derive(FromForm)]
    pub struct User<'a> {
        name: &'a RawStr,
        nickname: String,
    }

    impl<'a> UriDisplay<Query> for User<'a> {
        fn fmt(&self, f: &mut Formatter<Query>) -> fmt::Result {
            f.write_named_value("name", &self.name)?;
            f.write_named_value("nickname", &self.nickname)
        }
    }

    impl<'a, 'b> FromUriParam<Query, (&'a str, &'b str)> for User<'a> {
        type Target = User<'a>;

        fn from_uri_param((name, nickname): (&'a str, &'b str)) -> User<'a> {
            User { name: name.into(), nickname: nickname.to_string() }
        }
    }

    #[post("/<name>?<user..>")]
    pub fn person(name: &RawStr, user: Form<User>) -> String {
        format!(
            "name={}, name={}, nickname={}",
            name.to_string(),
            user.name.to_string(),
            user.nickname,
        )
    }

    #[get("/list")]
    pub fn list() -> String {
        let uri = uri!(person: name = "hey", user = ("Robert Mike", "Bob"));

        format!(
            "uri.path={}, uri.query={}",
            uri.path(),
            match uri.query() {
                Some(q) => q,
                None => "?",
            },
        )
    }
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/id", routes![id::post, id::json])
        .mount("/responder", routes![responder::index])
        .mount("/task", routes![task::id])
        .mount("/template", routes![template::index])
        .mount("/uri", routes![uri::person, uri::list, uri::add_user])
        .mount("/typed_uri", routes![typed_uri::person, typed_uri::list])
        .launch();
}

