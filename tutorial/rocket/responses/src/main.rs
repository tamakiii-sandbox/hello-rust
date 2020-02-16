#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

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

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/id", routes![id::post, id::json])
        .mount("/responder", routes![responder::index])
        .launch();
}

