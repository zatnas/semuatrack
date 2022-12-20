use super::schema::*;
use bigdecimal::BigDecimal;
use diesel::prelude::Queryable;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Cashflow {
    pub id: i64,
    pub category_id: Option<i16>,
    pub datetime: i64,
    pub amount: BigDecimal,
    pub note: Option<String>,
    pub place: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = cashflow)]
pub struct NewCashflow<'r> {
    pub datetime: i64,
    pub amount: &'r BigDecimal,
    pub note: Option<&'r str>,
    pub place: Option<&'r str>,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Category {
    pub id: i16,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = category)]
pub struct NewCategory<'r> {
    pub name: &'r str,
    pub color: Option<&'r str>,
    pub icon: Option<&'r str>,
}
