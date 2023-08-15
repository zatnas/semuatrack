#[macro_use]
extern crate diesel;

use actix_web::{error, get, middleware, web, App, HttpResponse, HttpServer, Responder};
use diesel::{prelude::*, r2d2};
use env_logger::Env;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

async fn read_account(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let accounts = web::block(move || {
        let mut conn = pool.get()?;

        actions::read_all_account(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(accounts))
}

async fn add_account(
    pool: web::Data<DbPool>,
    json: web::Json<models::JsonAccount>,
) -> actix_web::Result<impl Responder> {
    let color = match json.color.clone() {
        Some(c) => c,
        None => "255,255,255".to_string(),
    };
    let account = web::block(move || {
        let mut conn = pool.get()?;

        actions::insert_new_account(&mut conn, json.name.clone(), color.to_string())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(account))
}

async fn read_category(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let categories = web::block(move || {
        let mut conn = pool.get()?;

        actions::read_all_category(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(categories))
}

async fn add_category(
    pool: web::Data<DbPool>,
    json: web::Json<models::JsonCategory>,
) -> actix_web::Result<impl Responder> {
    let category = web::block(move || {
        let mut conn = pool.get()?;

        actions::insert_new_category(
            &mut conn,
            json.parent_id.clone(),
            json.name.clone(),
            json.color.clone(),
            json.icon.clone(),
        )
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(category))
}

async fn read_transaction(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let transactions = web::block(move || {
        let mut conn = pool.get()?;

        actions::read_recent_transaction(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(transactions))
}

async fn add_transaction(
    pool: web::Data<DbPool>,
    json: web::Json<models::JsonTransaction>,
) -> actix_web::Result<impl Responder> {
    let transaction = web::block(move || {
        let mut conn = pool.get()?;

        actions::insert_new_transaction(
            &mut conn,
            json.account_id,
            json.category_id,
            json.datetime,
            json.amount,
            json.labels.clone(),
        )
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(transaction))
}

async fn add_label(
    pool: web::Data<DbPool>,
    json: web::Json<models::JsonLabel>,
) -> actix_web::Result<impl Responder> {
    let account = web::block(move || {
        let mut conn = pool.get()?;

        actions::insert_new_label(&mut conn, json.name.clone(), json.color.clone())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(account))
}

async fn read_label(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let labels = web::block(move || {
        let mut conn = pool.get()?;

        actions::read_all_label(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(labels))
}

fn api_v1_scope_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/transaction")
            .route(web::get().to(read_transaction))
            .route(web::post().to(add_transaction)),
    )
    .service(
        web::resource("/account")
            .route(web::get().to(read_account))
            .route(web::post().to(add_account)),
    )
    .service(
        web::resource("/labels")
            .route(web::get().to(read_label))
            .route(web::post().to(add_label)),
    )
    .service(
        web::resource("/category")
            .route(web::get().to(read_category))
            .route(web::post().to(add_category)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let pool = initialize_db_pool();

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(web::scope("/api/v1").configure(api_v1_scope_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to PostgreSQL URL")
}
