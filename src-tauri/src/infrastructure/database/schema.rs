// @generated automatically by Diesel CLI.

diesel::table! {
    theme_settings (id) {
        id -> Integer,
        theme_name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
