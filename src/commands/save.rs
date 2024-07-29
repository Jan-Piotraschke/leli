use rusqlite::{params, Connection, Result};

pub fn save_html_metadata_to_db(html_files: &[String], db_path: &str) -> Result<()> {
    let mut conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS html_metadata (
                id INTEGER PRIMARY KEY,
                file_path TEXT NOT NULL
        )",
        [],
    )?;

    let tx = conn.transaction()?;
    for file_path in html_files {
        tx.execute(
            "INSERT INTO html_metadata (file_path) VALUES (?1)",
            params![file_path],
        )?;
    }
    tx.commit()?;

    println!("Saved HTML metadata to {}", db_path);
    Ok(())
}
