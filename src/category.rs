use crate::models::Category;
use crate::models::NewCategory;
use diesel::prelude::*;
use rocket::serde::{json::Json, Deserialize};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec, JsonSchema};
use semuatrack::establish_connection;

#[derive(Deserialize, JsonSchema)]
struct CategoryJson<'r> {
    name: &'r str,
    color: Option<&'r str>,
    icon: Option<&'r str>,
}

#[openapi]
#[get("/")]
fn read_category() -> Option<Json<Vec<Category>>> {
    let connection = &mut establish_connection();
    let results = crate::schema::category::table
        .order(crate::schema::category::id.desc())
        .load::<Category>(connection)
        .expect("Error loading category");

    Some(Json(results))
}

#[openapi]
#[post("/", data = "<category>")]
fn create_category_json(category: Json<CategoryJson>) -> Option<Json<Vec<Category>>> {
    let new_item = NewCategory {
        name: category.name,
        color: category.color,
        icon: category.icon,
    };
    let connection = &mut establish_connection();
    let row = diesel::insert_into(crate::schema::category::table)
        .values(&new_item)
        .returning(crate::schema::category::all_columns)
        .get_results(connection)
        .expect("Error fetching results");

    Some(Json(row))
}

pub fn api_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: create_category_json, read_category]
}
