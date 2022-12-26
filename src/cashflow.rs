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
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec, JsonSchema};
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

#[derive(Deserialize, JsonSchema)]
struct CashflowJson<'r> {
    datetime: i64,
    amount: BigDecimal,
    note: Option<&'r str>,
    place: Option<&'r str>,
}

#[derive(Deserialize, JsonSchema)]
struct CashflowPatchJson<'r> {
    datetime: Option<i64>,
    amount: Option<f32>,
    note: Option<&'r str>,
    place: Option<&'r str>,
}

#[openapi]
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

#[openapi]
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

#[openapi]
#[get("/")]
fn read_cashflow_api() -> Option<Json<Vec<Cashflow>>> {
    let connection = &mut establish_connection();
    let results = crate::schema::cashflow::table
        .order(crate::schema::cashflow::id.desc())
        .load::<Cashflow>(connection)
        .expect("Error loading cashflow");

    Some(Json(results))
}

#[openapi]
#[patch("/<cashflow_id>", data = "<cashflow>")]
fn update_cashflow_id_api(
    cashflow_id: i64,
    cashflow: Json<CashflowPatchJson>,
) -> Option<Json<Vec<Cashflow>>> {
    let binding = BigDecimal::from_f32(match cashflow.amount {
        Some(i) => i,
        None => 0f32,
    })
    .unwrap();
    let amount = match cashflow.amount {
        Some(_) => Some(&binding),
        None => None,
    };
    let user_patch = PatchCashflow {
        datetime: cashflow.datetime,
        amount,
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

pub fn api_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        settings: read_cashflow_api,
        create_cashflow_api,
        get_cashflow_id,
        update_cashflow_id_api
    ]
}
