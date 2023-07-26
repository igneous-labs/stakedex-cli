use lazy_static::lazy_static;
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use solana_sdk::signature::Signature;
use std::{error::Error, include_str, path::Path, str::FromStr};

pub mod schema;
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

/// Returns None if db empty
pub fn earliest_indexed_signature(conn: &Connection) -> Result<Option<Signature>, Box<dyn Error>> {
    let sig: String = match conn.query_row(
        "SELECT sig FROM invocations ORDER BY slot ASC LIMIT 1",
        [],
        |row| row.get(0),
    ) {
        Ok(r) => r,
        Err(e) => {
            if let rusqlite::Error::QueryReturnedNoRows = e {
                return Ok(None);
            } else {
                return Err(e.into());
            }
        }
    };
    Ok(Some(Signature::from_str(&sig)?))
}

/// Returns None if db empty
pub fn latest_indexed_signature(conn: &Connection) -> Result<Option<Signature>, Box<dyn Error>> {
    let sig: String = match conn.query_row(
        "SELECT sig FROM invocations ORDER BY slot DESC LIMIT 1",
        [],
        |row| row.get(0),
    ) {
        Ok(r) => r,
        Err(e) => {
            if let rusqlite::Error::QueryReturnedNoRows = e {
                return Ok(None);
            } else {
                return Err(e.into());
            }
        }
    };
    Ok(Some(Signature::from_str(&sig)?))
}

#[cfg(test)]
pub mod tests {
    use super::{test_utils::create_test_db, *};

    #[test]
    fn test_migrations() {
        let conn = create_test_db();

        conn.execute(
            "INSERT INTO invocations
            (sig, signer, ix, unix_timestamp, slot, amount_in, amount_out, mint_in, mint_out)
            VALUES
            (:sig, :signer, :ix, :unix_timestamp, :slot, :amount_in, :amount_out, :mint_in, :mint_out)"
            ,&[
                (":sig", "abc"),
                (":signer", "def"),
                (":ix", "1"),
                (":unix_timestamp", "2"),
                (":slot", "2"),
                (":amount_in", "123"),
                (":amount_out", "456"),
                (":mint_in", "ghi"),
                (":mint_out", "jkl"),
            ]).unwrap();
    }

    const TEST_SIG: &str =
        "5XgPzWKZSaC8phfRPDG55MMgxaDb35iNRfnPQEbd76nehutdKYU4Stp1ChKZtrjpQYSVZqs9az4p4RootDUwx8Ct";

    #[test]
    fn test_earliest_indexed_signature_empty() {
        let conn = create_test_db();
        let actual = earliest_indexed_signature(&conn).unwrap();
        assert!(actual.is_none());
    }

    #[test]
    fn test_earliest_indexed_signature() {
        let conn = create_test_db();

        conn.execute(
            "INSERT INTO invocations
            (sig, signer, ix, unix_timestamp, slot, amount_in, amount_out, mint_in, mint_out)
            VALUES
            (:sig, :signer, :ix, :unix_timestamp, :slot, :amount_in, :amount_out, :mint_in, :mint_out)"
            ,&[
                (":sig", TEST_SIG),
                (":signer", "def"),
                (":ix", "1"),
                (":unix_timestamp", "2"),
                (":slot", "206389108"),
                (":amount_in", "123"),
                (":amount_out", "456"),
                (":mint_in", "ghi"),
                (":mint_out", "jkl"),
            ]).unwrap();

        conn.execute(
            "INSERT INTO invocations
            (sig, signer, ix, unix_timestamp, slot, amount_in, amount_out, mint_in, mint_out)
            VALUES
            (:sig, :signer, :ix, :unix_timestamp, :slot, :amount_in, :amount_out, :mint_in, :mint_out)"
            ,&[
                (":sig", "abc"),
                (":signer", "def"),
                (":ix", "1"),
                (":unix_timestamp", "3"),
                (":slot", "206389109"),
                (":amount_in", "123"),
                (":amount_out", "456"),
                (":mint_in", "ghi"),
                (":mint_out", "jkl"),
            ]).unwrap();
        let actual = earliest_indexed_signature(&conn).unwrap().unwrap();
        assert_eq!(actual.to_string(), TEST_SIG);
    }
}
