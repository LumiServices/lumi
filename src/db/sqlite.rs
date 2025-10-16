use std::path::PathBuf;
use rusqlite::{params, Connection};

pub fn open_db(path: &PathBuf) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    Ok(conn)
}

pub fn insert(conn: &Connection, table_name: &str, key_column: &str, value_column: &str, key: &[u8], value: &[u8]) -> rusqlite::Result<()> {
    let sql = format!(
        "INSERT INTO {} ({}, {}) VALUES (?1, ?2)",
        table_name, key_column, value_column
    );
    conn.execute(&sql, params![key, value])?;
    Ok(())
}

pub fn get(conn: &Connection, table_name: &str, key_column: &str, value_column: &str, key: &[u8]) -> rusqlite::Result<Option<Vec<u8>>> {
    let sql: String = format!(
        "SELECT {} FROM {} WHERE {} = ?1",
        value_column, table_name, key_column
    );
    let result = conn.query_row(&sql, params![key], |row| row.get(0));
    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }

}