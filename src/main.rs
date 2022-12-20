#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
use std::env;

mod cashflow;
mod category;
mod models;
mod schema;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/cashflow", cashflow::cashflow_ui()))
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index,])
        .mount("/cashflow", cashflow::route())
        .mount("/api/cashflow", cashflow::api_route())
        .mount("/api/category", category::api_route())
        .attach(Template::fairing())
}
