use serde::{Deserialize, Serialize};

use crate::schema::account;
use crate::schema::category;
use crate::schema::label;
use crate::schema::transaction;
use crate::schema::transaction_label;

#[derive(Debug, Serialize, Queryable)]
#[diesel(table_name = account)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = account)]
pub struct NewAccount {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Deserialize)]
pub struct JsonAccount {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Queryable)]
#[diesel(table_name = category)]
pub struct Category {
    pub id: i16,
    pub parent_id: Option<i16>,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = category)]
pub struct NewCategory {
    pub parent_id: Option<i16>,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JsonCategory {
    pub parent_id: Option<i16>,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Queryable, Associations)]
#[diesel(belongs_to(Account))]
#[diesel(table_name = transaction)]
pub struct Transaction {
    pub id: i64,
    pub account_id: i32,
    pub category_id: i16,
    pub datetime: i64,
    pub amount: bigdecimal::BigDecimal,
    pub note: Option<String>,
    pub place: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = transaction)]
pub struct NewTransaction {
    pub account_id: Option<i32>,
    pub category_id: Option<i16>,
    pub datetime: i64,
    pub amount: bigdecimal::BigDecimal,
}

#[derive(Debug, Deserialize)]
pub struct JsonTransaction {
    pub account_id: Option<i32>,
    pub category_id: Option<i16>,
    pub datetime: Option<i64>,
    pub amount: f32,
    pub labels: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Queryable)]
#[diesel(table_name = label)]
pub struct Label {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = label)]
pub struct NewLabel {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Deserialize)]
pub struct JsonLabel {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize, Queryable, Associations)]
#[diesel(table_name = transaction_label)]
#[diesel(belongs_to(Transaction))]
#[diesel(belongs_to(Label))]
#[diesel(primary_key(transaction_id, label_id))]
pub struct TransactionLabel {
    pub transaction_id: i64,
    pub label_id: i64,
}

#[derive(Debug, Serialize)]
pub struct JsonTransactionLabel {
    pub id: i64,
    pub account_id: i32,
    pub category_id: i16,
    pub datetime: i64,
    pub amount: bigdecimal::BigDecimal,
    pub note: Option<String>,
    pub place: Option<String>,
    pub labels: Vec<Label>,
}
