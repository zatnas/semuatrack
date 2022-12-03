use super::schema::*;
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
