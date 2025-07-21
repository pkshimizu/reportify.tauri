use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::infrastructure::database::schema::settings;

#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = settings)]
pub struct Settings {
    pub id: i32,
    pub theme: String,
    pub language: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = settings)]
pub struct NewSettings<'a> {
    pub theme: &'a str,
    pub language: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = settings)]
pub struct UpdateSettings<'a> {
    pub theme: Option<&'a str>,
    pub language: Option<&'a str>,
    pub updated_at: NaiveDateTime,
}
