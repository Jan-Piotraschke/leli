// src/commands/models.rs
use diesel::prelude::*;
use crate::schema::html_metadata;

#[derive(Queryable, Insertable)]
#[diesel(table_name = html_metadata)]
pub struct HtmlMetadata {
    pub id: Option<i32>,
    pub file_path: String,
}
