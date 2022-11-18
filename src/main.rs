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
struct TransactionPost<'r> {
    datetime: i32,
    amount: f32,
    #[field(default = None)]
    note: Option<&'r str>,
    #[field(default = None)]
    place: Option<&'r str>,
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(transaction_ui()))
}

#[get("/transaction/<id>")]
fn get_transaction_id(id: u16) -> String {
    format!("Get transaction of id {}", id)
}

#[get("/transaction")]
fn transaction_ui() -> Template {
    Template::render(
        "transaction",
        context!(
        title: "Transaction"
        ),
    )
}

#[get("/transaction")]
fn get_transaction_all() -> Option<Json<Vec<Transaction>>> {
    let results = transactions::table
        .order(transactions::id.desc())
        .load::<Transaction>(&mut establish_connection())
        .expect("Error loading transactions");

    Some(Json(results))
}

#[post("/transaction", data = "<transaction>", rank = 3)]
fn create_transaction_json(transaction: Json<TransactionPost>) {
    println!("{}", transaction.amount)
}

#[post("/transaction", data = "<transaction>", rank = 2)]
fn create_transaction_form(transaction: Form<TransactionPost>) -> Redirect {
    println!("Add new transaction: {}", transaction.amount);
    let new_transaction = TransactionCreate {
        datetime: transaction.datetime,
        amount: transaction.amount,
        note: transaction.note,
        place: transaction.place,
    };
    let _insert = diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .execute(&mut establish_connection())
        .expect("Error creating new transaction");
    Redirect::to(uri!(transaction_ui()))
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, transaction_ui, create_transaction_form])
        .mount("/", FileServer::from(relative!("static")))
        .mount(
            "/api",
            routes![
                get_transaction_id,
                get_transaction_all,
                create_transaction_json,
            ],
        )
        .attach(Template::fairing())
}
