use crate::models::Category;
use crate::models::NewCategory;
use diesel::prelude::*;
use rocket::serde::{json::Json, Deserialize};
use semuatrack::establish_connection;

#[derive(Deserialize)]
struct CategoryJson<'r> {
    name: &'r str,
    color: Option<&'r str>,
    icon: Option<&'r str>,
}

#[get("/")]
fn read_category() -> Option<Json<Vec<Category>>> {
    let connection = &mut establish_connection();
    let results = crate::schema::category::table
        .order(crate::schema::category::id.desc())
        .load::<Category>(connection)
        .expect("Error loading category");

    Some(Json(results))
}

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

pub fn api_route() -> Vec<rocket::Route> {
    routes![create_category_json, read_category]
}
