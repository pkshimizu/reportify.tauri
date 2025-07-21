// @generated automatically by Diesel CLI.

diesel::table! {
    settings (id) {
        id -> Integer,
        theme -> Text,
        language -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
