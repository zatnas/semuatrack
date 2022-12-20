// @generated automatically by Diesel CLI.

diesel::table! {
    cashflow (id) {
        id -> Int8,
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
