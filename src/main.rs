#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::form::{Context, Form};
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
struct TransactionPost {
    amount: f32,
    note: String,
    place: String,
}

#[derive(FromForm)]
struct PostExample {
    amount: i64,
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(transaction_ui()))
}

#[post("/", data = "<_post_example>")]
fn submit(_post_example: Form<PostExample>) -> Template {
    println!("{}", _post_example.amount);
    Template::render("index", &Context::default())
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
    let transaction = transaction.clone();
    let new_transaction = TransactionCreate {
        amount: transaction.amount,
        note: Some(transaction.note),
        place: Some(transaction.place),
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
        .mount(
            "/",
            routes![index, submit, transaction_ui, create_transaction_form],
        )
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
