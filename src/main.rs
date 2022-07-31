#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::form::{Context, Form};
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize};
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
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

#[derive(Deserialize, FromForm)]
struct TransactionPost {
    value: f32,
}

#[derive(FromForm)]
struct PostExample {
    value: i64,
}

#[get("/")]
fn index() -> Template {
    // "Application successfully started"
    Template::render("index", &Context::default())
}

#[post("/", data = "<_post_example>")]
fn submit(_post_example: Form<PostExample>) -> Template {
    println!("{}", _post_example.value);
    Template::render("index", &Context::default())
}

#[get("/transaction/<id>")]
fn get_transaction_id(id: u16) -> String {
    format!("Get transaction of id {}", id)
}

#[get("/transaction")]
fn get_transaction_all() -> Option<Json<Vec<Transaction>>> {
    let results = transactions::table
        .load::<Transaction>(&mut establish_connection())
        .expect("Error loading transactions");

    Some(Json(results))
}

#[post("/transaction", data = "<_transaction>", rank = 3)]
fn create_transaction_json(_transaction: Json<TransactionPost>) {
    println!("{}", _transaction.value)
}

#[post("/transaction", data = "<_transaction>", rank = 2)]
fn create_transaction_form(_transaction: Form<TransactionPost>) -> Redirect {
    println!("Add new transaction{}", _transaction.value);
    let new_transaction = TransactionCreate {
        amount: _transaction.value,
    };
    let _insert = diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .execute(&mut establish_connection())
        .expect("Error creating new transaction");
    Redirect::to(uri!(index))
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, submit,])
        .mount(
            "/api",
            routes![
                get_transaction_id,
                get_transaction_all,
                create_transaction_json,
                create_transaction_form,
            ],
        )
        .attach(Template::fairing())
}
