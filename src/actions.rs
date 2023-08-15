use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn read_all_account(conn: &mut PgConnection) -> Result<Vec<models::Account>, DbError> {
    use crate::schema::account::dsl::*;

    let accounts = account.load::<models::Account>(conn)?;
    Ok(accounts)
}

pub fn insert_new_account(
    conn: &mut PgConnection,
    name: String,
    color: String,
) -> Result<models::Account, DbError> {
    use crate::schema::account::dsl;

    let new_account = models::NewAccount { name, color };
    let result_account = diesel::insert_into(dsl::account)
        .values(&new_account)
        .get_result(conn)?;

    Ok(result_account)
}

pub fn read_all_category(conn: &mut PgConnection) -> Result<Vec<models::Category>, DbError> {
    use crate::schema::category::dsl::*;

    let categories = category.load::<models::Category>(conn)?;
    Ok(categories)
}

pub fn insert_new_category(
    conn: &mut PgConnection,
    parent_id: Option<i16>,
    name: String,
    color: Option<String>,
    icon: Option<String>,
) -> Result<models::Category, DbError> {
    use crate::schema::category::dsl;

    let new_category = models::NewCategory {
        parent_id,
        name,
        color,
        icon,
    };
    let result_category = diesel::insert_into(dsl::category)
        .values(&new_category)
        .get_result(conn)?;

    Ok(result_category)
}

pub fn read_all_label(conn: &mut PgConnection) -> Result<Vec<models::Label>, DbError> {
    use crate::schema::label::dsl::*;

    let labels = label.load::<models::Label>(conn)?;
    Ok(labels)
}

pub fn insert_new_label(
    conn: &mut PgConnection,
    name: String,
    color: String,
) -> Result<models::Label, DbError> {
    use crate::schema::label::dsl;

    let new_label = models::NewLabel { name, color };
    let result_label = diesel::insert_into(dsl::label)
        .values(&new_label)
        .get_result(conn)?;

    Ok(result_label)
}

pub fn read_recent_transaction(
    conn: &mut PgConnection,
) -> Result<Vec<models::Transaction>, DbError> {
    use crate::schema::transaction::dsl::*;

    let transactions = transaction
        .limit(20)
        .order_by(datetime.desc())
        .get_results(conn)?;
    Ok(transactions)
}

pub fn insert_new_transaction(
    conn: &mut PgConnection,
    account_id: Option<i32>,
    category_id: Option<i16>,
    datetime: Option<i64>,
    amount: f32,
    labels: Option<Vec<String>>,
) -> Result<models::JsonTransactionLabel, DbError> {
    use crate::schema::label::dsl as ldsl;
    use crate::schema::transaction::dsl;

    let amount = BigDecimal::from_f32(amount).unwrap();
    let datetime_now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    let datetime = match datetime {
        Some(d) => d,
        None => datetime_now,
    };

    let result_label: Vec<models::Label> = if let Some(lbls) = labels {
        let new_labels: Vec<models::NewLabel> = lbls
            .iter()
            .map(|l| models::NewLabel {
                name: l.clone(),
                color: "Default".to_string(),
            })
            .collect();
        diesel::insert_into(ldsl::label)
            .values(&new_labels)
            .get_results(conn)?
    } else {
        vec![]
    };

    let new_transaction = models::NewTransaction {
        account_id,
        category_id,
        amount,
        datetime,
    };
    let result_transaction = diesel::insert_into(dsl::transaction)
        .values(&new_transaction)
        .get_result(conn)?;

    let result = models::JsonTransactionLabel {
        labels: result_label,
        ..result_transaction
    };

    Ok(result)
}
