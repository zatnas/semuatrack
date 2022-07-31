table! {
    transactions (id) {
        id -> Integer,
        datetime -> Nullable<Integer>,
        amount -> Float,
        note -> Nullable<Text>,
        place -> Nullable<Text>,
    }
}
