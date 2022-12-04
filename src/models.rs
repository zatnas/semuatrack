use super::schema::*;
use diesel::prelude::Queryable;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Cashflow {
    pub id: i32,
    pub category_id: Option<i32>,
    pub datetime: i32,
    pub amount: f32,
    pub note: Option<String>,
    pub place: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = cashflow)]
pub struct NewCashflow<'r> {
    pub datetime: i32,
    pub amount: f32,
    pub note: Option<&'r str>,
    pub place: Option<&'r str>,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = category)]
pub struct NewCategory {
    pub name: String,
    pub color: String,
    pub icon: String,
}
