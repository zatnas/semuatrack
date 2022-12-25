use crate::models::Cashflow;
use crate::models::NewCashflow;
use crate::models::PatchCashflow;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use diesel::prelude::*;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize};
use rocket_dyn_templates::{context, Template};
use semuatrack::establish_connection;

#[derive(FromForm, Deserialize)]
struct CashflowPost<'r> {
    datetime: i64,
    amount: f32,
    #[field(default = None)]
    note: Option<&'r str>,
    #[field(default = None)]
    place: Option<&'r str>,
}

#[derive(Deserialize)]
struct CashflowJson<'r> {
    datetime: i64,
    amount: BigDecimal,
    note: Option<&'r str>,
    place: Option<&'r str>,
}

#[derive(Deserialize)]
struct CashflowPatchJson<'r> {
    datetime: Option<i64>,
    amount: Option<f32>,
    note: Option<&'r str>,
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

#[post("/", data = "<cashflow>")]
fn create_cashflow_api(cashflow: Json<CashflowJson>) -> Option<Json<Vec<Cashflow>>> {
    let new_cashflow = NewCashflow {
        datetime: cashflow.datetime,
        amount: &cashflow.amount,
        note: cashflow.note,
        place: cashflow.place,
    };
    let connection = &mut establish_connection();
    let row = diesel::insert_into(crate::schema::cashflow::table)
        .values(&new_cashflow)
        .returning(crate::schema::cashflow::all_columns)
        .get_results(connection)
        .expect("Error creating new cashflow");

    Some(Json(row))
}

#[get("/")]
fn read_cashflow_api() -> Option<Json<Vec<Cashflow>>> {
    let connection = &mut establish_connection();
    let results = crate::schema::cashflow::table
        .order(crate::schema::cashflow::id.desc())
        .load::<Cashflow>(connection)
        .expect("Error loading cashflow");

    Some(Json(results))
}

#[patch("/<cashflow_id>", data = "<cashflow>")]
fn update_cashflow_id_api(cashflow_id: i64, cashflow: Json<CashflowPatchJson>) -> Option<Json<Vec<Cashflow>>> {
    let amount = match cashflow.amount {
        Some(i) => BigDecimal::from_f32(i),
        None => None
    }.unwrap();
    let user_patch = PatchCashflow {
        datetime: cashflow.datetime,
        amount: Some(&amount),
        note: cashflow.note,
        place: cashflow.place,
    };
    let connection = &mut establish_connection();
    let query = diesel::update(crate::schema::cashflow::table)
    .filter(crate::schema::cashflow::dsl::id.eq(cashflow_id))
        .set(&user_patch)
        .returning(crate::schema::cashflow::all_columns)
        .get_results(connection)
        .expect("Error updating cashflow");

    Some(Json(query))
}

#[post("/", data = "<cashflow>")]
fn create_cashflow_form(cashflow: Form<CashflowPost>) -> Redirect {
    let amount = BigDecimal::from_f32(cashflow.amount).unwrap();
    let new_cashflow = NewCashflow {
        datetime: cashflow.datetime,
        amount: &amount,
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
    routes![read_cashflow_api, create_cashflow_api, get_cashflow_id, update_cashflow_id_api]
}
