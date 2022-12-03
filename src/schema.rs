// @generated automatically by Diesel CLI.

diesel::table! {
    cashflow (id) {
        id -> Integer,
        category_id -> Nullable<Integer>,
        datetime -> Integer,
        amount -> Float,
        note -> Nullable<Text>,
        place -> Nullable<Text>,
    }
}

diesel::table! {
    category (id) {
        id -> Integer,
        name -> Text,
        color -> Nullable<Text>,
        icon -> Nullable<Text>,
    }
}

diesel::joinable!(cashflow -> category (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    cashflow,
    category,
);
