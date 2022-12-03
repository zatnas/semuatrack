// @generated automatically by Diesel CLI.

diesel::table! {
    cashflow (id) {
        id -> Integer,
        datetime -> Integer,
        amount -> Float,
        note -> Nullable<Text>,
        place -> Nullable<Text>,
    }
}
