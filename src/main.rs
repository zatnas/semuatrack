#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
use rocket_okapi::mount_endpoints_and_merged_docs;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

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
    let swagger_ui_config = SwaggerUIConfig {
        url: "../api/openapi.json".to_string(),
        ..Default::default()
    };
    let mut rocket_build = rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index,])
        .mount("/cashflow", cashflow::route())
        .mount("/swagger", make_swagger_ui(&swagger_ui_config))
        .attach(Template::fairing());

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        rocket_build,
        "/api".to_string(),
        openapi_settings,
        "/api/cashflow" => cashflow::api_routes_and_docs(&openapi_settings),
        "/api/category" => category::api_routes_and_docs(&openapi_settings),
    }

    rocket_build
}
