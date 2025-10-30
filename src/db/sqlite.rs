use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use rusqlite::{params, Connection};

pub static DB: OnceLock<Database> = OnceLock::new();

#[derive(Debug)]
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &PathBuf) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Database { conn: Mutex::new(conn) })
    }

 pub fn insert(&self, table_name: &str, key_column: &str, value_column: &str, key: &[u8], value: &[u8]) -> rusqlite::Result<()> {
    let sql = format!(
        "INSERT INTO {} ({}, {}) VALUES (?1, ?2)",
        table_name, key_column, value_column
     );
    let conn = self.conn.lock().unwrap();
    conn.execute(&sql, params![key, value])?;
    Ok(())
}
pub fn create_table(&self, table_name: &str, key_column: &str, value_column: &str) -> rusqlite::Result<()> {
    let sql = format!(
        "CREATE TABLE IF NOT EXISTS {} ({} BLOB PRIMARY KEY, {} BLOB)",
        table_name, key_column, value_column
    );
    let conn = self.conn.lock().unwrap();
    conn.execute(&sql, [])?;
    Ok(())
}
pub fn get(&self, table_name: &str, key_column: &str, value_column: &str, key: &[u8]) -> rusqlite::Result<Option<Vec<u8>>> {
    let sql = format!(
        "SELECT {} FROM {} WHERE {} = ?1",
        value_column, table_name, key_column
    );
    let conn = self.conn.lock().unwrap();
    let result = conn.query_row(&sql, params![key], |row| row.get(0));
    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
 }
pub fn delete(&self, table_name: &str, key_column: &str, key: &[u8]) -> rusqlite::Result<()> {
    let sql = format!(
        "DELETE FROM {} WHERE {} = ?1",
        table_name, key_column
    );
    let conn = self.conn.lock().unwrap();
    conn.execute(&sql, params![key])?;
    Ok(())
}
pub fn update(&self, table_name: &str, value_column: &str, new_value: &[u8], key_column: &str, key: &[u8]) -> rusqlite::Result<usize> {
    let sql = format!(
        "UPDATE {} SET {} = ?1 WHERE {} = ?2",
        table_name, value_column, key_column
    );
    let conn = self.conn.lock().unwrap();
    conn.execute(&sql, params![new_value, key])
}
}