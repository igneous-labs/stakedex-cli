#![cfg(test)]

use rusqlite::Connection;

use super::MIGRATIONS;

pub fn create_test_db() -> Connection {
    let mut conn = Connection::open_in_memory().unwrap();
    conn.pragma_update(None, "journal_mode", &"WAL").unwrap();
    MIGRATIONS.to_latest(&mut conn).unwrap();
    conn
}
