#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize};
use rocket::{Build, Rocket};
use rocket_dyn_templates::{context, Template};
use std::env;

mod models;
mod schema;

use models::*;
use schema::*;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(FromForm, Deserialize, Clone)]
struct CashflowPost<'r> {
    datetime: i32,
    amount: f32,
    #[field(default = None)]
    note: Option<&'r str>,
    #[field(default = None)]
    place: Option<&'r str>,
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(cashflow_ui()))
}

#[get("/cashflow/<id>")]
fn get_cashflow_id(id: u16) -> String {
    format!("Get cashflow of id {}", id)
}

#[get("/cashflow")]
fn cashflow_ui() -> Template {
    Template::render(
        "cashflow",
        context!(
        title: "Cashflow"
        ),
    )
}

#[get("/cashflow")]
fn get_cashflow_all() -> Option<Json<Vec<Cashflow>>> {
    let results = cashflow::table
        .order(cashflow::id.desc())
        .load::<Cashflow>(&mut establish_connection())
        .expect("Error loading cashflow");

    Some(Json(results))
}

#[post("/cashflow", data = "<cashflow>", rank = 3)]
fn create_cashflow_json(cashflow: Json<CashflowPost>) {
    println!("{}", cashflow.amount)
}

#[post("/cashflow", data = "<cashflow>", rank = 2)]
fn create_cashflow_form(cashflow: Form<CashflowPost>) -> Redirect {
    println!("Add new cashflow: {}", cashflow.amount);
    let new_cashflow = NewCashflow {
        datetime: cashflow.datetime,
        amount: cashflow.amount,
        note: cashflow.note,
        place: cashflow.place,
    };
    let _insert = diesel::insert_into(cashflow::table)
        .values(&new_cashflow)
        .execute(&mut establish_connection())
        .expect("Error creating new cashflow");
    Redirect::to(uri!(cashflow_ui()))
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, cashflow_ui, create_cashflow_form])
        .mount("/", FileServer::from(relative!("static")))
        .mount(
            "/api",
            routes![get_cashflow_id, get_cashflow_all, create_cashflow_json,],
        )
        .attach(Template::fairing())
}
