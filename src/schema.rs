// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int4,
        name -> Text,
        color -> Text,
    }
}

diesel::table! {
    cashflow (id) {
        id -> Int8,
        account_id -> Nullable<Int4>,
        category_id -> Nullable<Int2>,
        datetime -> Int8,
        amount -> Numeric,
        note -> Nullable<Text>,
        place -> Nullable<Text>,
    }
}

diesel::table! {
    category (id) {
        id -> Int2,
        parent_id -> Nullable<Int2>,
        name -> Text,
        color -> Nullable<Text>,
        icon -> Nullable<Text>,
    }
}

diesel::table! {
    tag (id) {
        id -> Int4,
        name -> Text,
        color -> Text,
    }
}

diesel::joinable!(cashflow -> account (account_id));
diesel::joinable!(cashflow -> category (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    cashflow,
    category,
    tag,
);
