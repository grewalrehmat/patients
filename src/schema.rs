// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        role -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}
