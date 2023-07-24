#![cfg(test)]

use rusqlite::Connection;

use super::MIGRATIONS;

pub fn create_test_db() -> Connection {
    let mut conn = Connection::open_in_memory().unwrap();
    conn.pragma_update(None, "journal_mode", &"WAL").unwrap();
    MIGRATIONS.to_latest(&mut conn).unwrap();
    conn
}

/*
mod gen_example {
    use std::str::FromStr;
    use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
    use solana_sdk::{signature::Signature, commitment_config::CommitmentConfig};
    use solana_transaction_status::{UiTransactionEncoding, EncodedConfirmedTransactionWithStatusMeta};

    #[test]
    fn gen_test_examples() {
        let client = RpcClient::new("https://api.mainnet-beta.solana.com");
        let tx = client.get_transaction_with_config(
            &Signature::from_str("5XgPzWKZSaC8phfRPDG55MMgxaDb35iNRfnPQEbd76nehutdKYU4Stp1ChKZtrjpQYSVZqs9az4p4RootDUwx8Ct").unwrap(),
            RpcTransactionConfig {
                encoding: Some(UiTransactionEncoding::Base64),
                commitment: Some(CommitmentConfig::finalized()),
                max_supported_transaction_version: Some(0),
            },
        ).unwrap();
        let json = serde_json::to_string(&tx).unwrap();
        let _round_trip: EncodedConfirmedTransactionWithStatusMeta = serde_json::from_str(&json).unwrap();
        println!("{json:}");
    }
}
*/
