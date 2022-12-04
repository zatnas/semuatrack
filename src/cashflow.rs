use crate::Cashflow;
use crate::NewCashflow;
use diesel::prelude::*;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize};
use rocket_dyn_templates::{context, Template};
use semuatrack::establish_connection;

#[derive(FromForm, Deserialize, Clone)]
struct CashflowPost<'r> {
    datetime: i32,
    amount: f32,
    #[field(default = None)]
    note: Option<&'r str>,
    #[field(default = None)]
    place: Option<&'r str>,
}

#[get("/<id>")]
fn get_cashflow_id(id: u16) -> String {
    format!("Get cashflow of id {}", id)
}

#[get("/")]
fn cashflow_ui() -> Template {
    Template::render(
        "cashflow",
        context!(
        title: "Cashflow"
        ),
    )
}

#[get("/")]
fn get_cashflow_all() -> Option<Json<Vec<Cashflow>>> {
    let connection = &mut establish_connection();
    let results = crate::schema::cashflow::table
        .order(crate::schema::cashflow::id.desc())
        .load::<Cashflow>(connection)
        .expect("Error loading cashflow");

    Some(Json(results))
}

#[post("/", data = "<cashflow>", rank = 3)]
fn create_cashflow_json(cashflow: Json<CashflowPost>) {
    println!("{}", cashflow.amount)
}

#[post("/", data = "<cashflow>", rank = 2)]
fn create_cashflow_form(cashflow: Form<CashflowPost>) -> Redirect {
    println!("Add new cashflow: {}", cashflow.amount);
    let new_cashflow = NewCashflow {
        datetime: cashflow.datetime,
        amount: cashflow.amount,
        note: cashflow.note,
        place: cashflow.place,
    };
    let connection = &mut establish_connection();
    let _insert = diesel::insert_into(crate::schema::cashflow::table)
        .values(&new_cashflow)
        .execute(connection)
        .expect("Error creating new cashflow");
    Redirect::to(uri!(cashflow_ui()))
}

pub fn route() -> Vec<rocket::Route> {
    routes![create_cashflow_form, cashflow_ui,]
}

pub fn api_route() -> Vec<rocket::Route> {
    routes![get_cashflow_all, create_cashflow_json, get_cashflow_id]
}
