// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int4,
        name -> Text,
        color -> Text,
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
    label (id) {
        id -> Int8,
        name -> Text,
        color -> Text,
    }
}

diesel::table! {
    transaction (id) {
        id -> Int8,
        account_id -> Int4,
        category_id -> Int2,
        datetime -> Int8,
        amount -> Numeric,
        note -> Nullable<Text>,
        place -> Nullable<Text>,
    }
}

diesel::table! {
    transaction_label (transaction_id, label_id) {
        transaction_id -> Int8,
        label_id -> Int8,
    }
}

diesel::joinable!(transaction -> account (account_id));
diesel::joinable!(transaction -> category (category_id));
diesel::joinable!(transaction_label -> label (label_id));
diesel::joinable!(transaction_label -> transaction (transaction_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    category,
    label,
    transaction,
    transaction_label,
);
