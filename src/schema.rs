table! {
    transactions (id) {
        id -> Integer,
        datetime -> Integer,
        amount -> Float,
        note -> Nullable<Text>,
        place -> Nullable<Text>,
    }
}
