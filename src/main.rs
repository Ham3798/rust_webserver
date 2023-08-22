#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::config::{Config, Environment};
use rocket::response::content;

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html("<h1>Hello, world!</h1>")
}

fn main() {
    let config = Config::build(Environment::Staging) // 또는 다른 환경: Development, Production
        .address("0.0.0.0")
        .port(80)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![index])
        .launch();
}
