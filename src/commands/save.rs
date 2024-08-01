// src/commands/save.rs
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;
use dotenvy::dotenv;
use std::env;
use crate::commands::models::HtmlMetadata;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn save_html_metadata_to_db(html_files: &[String], conn: &mut SqliteConnection) -> Result<(), Error> {
    use crate::schema::html_metadata::dsl::*;

    for path in html_files {
        let new_metadata = HtmlMetadata {
            id: None,
            file_path: path.clone(),
        };

        diesel::insert_into(html_metadata)
            .values(&new_metadata)
            .execute(conn)?; // Use mutable reference
    }

    println!("Saved HTML metadata to database");
    Ok(())
}
