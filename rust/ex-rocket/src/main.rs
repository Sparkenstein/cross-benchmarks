#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]


extern crate rocket;

use rocket::{get, routes};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
