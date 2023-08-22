#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content;

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html("<h1>Hello, world!</h1>")
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
