#![feature(proc_macro_hygiene, decl_macro)]

use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_contrib::json::Json;
use serde_json::json;
use std::io::Cursor;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

#[get("/")]
fn index() -> Result<String, ()> {
    Ok("hello world".to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    #[serde(rename = "casterId")]
    caster_id: String,
    #[serde(rename = "htmlResourceId")]
    html_resource_id: i32,
    #[serde(rename = "htmlUrl")]
    html_url: String,
}

impl<'r> Responder<'r> for Task {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(json!(self).to_string()))
            .header(ContentType::new("application", "json"))
            .ok()
    }
}

#[post("/post", format = "json", data = "<task>")]
fn post(task: Json<Task>) -> Result<Task, ()> {
    println!("{:?}", task);
    Ok(task.0)
}
fn main() {
    rocket::ignite().mount("/", routes![index, post]).launch();
}
