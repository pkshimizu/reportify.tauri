use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::infrastructure::database::schema::theme_settings;

#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = theme_settings)]
pub struct ThemeSetting {
    pub id: i32,
    pub theme_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = theme_settings)]
pub struct NewThemeSetting<'a> {
    pub theme_name: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = theme_settings)]
pub struct UpdateThemeSetting<'a> {
    pub theme_name: &'a str,
    pub updated_at: NaiveDateTime,
}