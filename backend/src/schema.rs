// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        password_hash -> Bytea,
    }
}
