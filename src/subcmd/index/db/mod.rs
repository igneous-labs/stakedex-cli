use lazy_static::lazy_static;
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use std::{include_str, path::Path};

pub mod test_utils;

lazy_static! {
    static ref MIGRATION_1_UP: &'static str = include_str!("schema/1_up.sql");
    static ref MIGRATION_1_DOWN: &'static str = include_str!("schema/1_down.sql");
    static ref MIGRATIONS: Migrations<'static> =
        Migrations::new(vec![M::up(&MIGRATION_1_UP).down(&MIGRATION_1_DOWN),]);
}

/// panics if any DB errors encountered
pub fn create_conn<P: AsRef<Path>>(path: P) -> Connection {
    let mut conn = Connection::open(path).unwrap();
    conn.pragma_update(None, "journal_mode", "WAL").unwrap();
    MIGRATIONS.to_latest(&mut conn).unwrap();
    conn
}

#[cfg(test)]
pub mod tests {
    use super::{test_utils::create_test_db, *};

    #[test]
    fn test_migrations() {
        let conn = create_test_db();

        conn.execute(
            "INSERT INTO invocations (sig, signer, ix, amount_in, amount_out, mint_in, mint_out) VALUES (:sig, :signer, :ix, :amount_in, :amount_out, :mint_in, :mint_out)"
            ,&[
                (":sig", "abc"),
                (":signer", "def"),
                (":ix", "1"),
                (":amount_in", "123"),
                (":amount_out", "456"),
                (":mint_in", "ghi"),
                (":mint_out", "jkl"),
            ]).unwrap();
    }
}
