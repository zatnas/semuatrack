use super::schema::*;
use diesel::prelude::*;
// use serde::Serialize;

#[derive(Debug, Queryable)]
pub struct Transaction {
    pub id: i32,
    pub datetime: Option<i32>,
    pub amount: f32,
    pub note: Option<String>,
    pub place: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = transactions)]
pub struct TransactionCreate {
    pub amount: f32,
}
