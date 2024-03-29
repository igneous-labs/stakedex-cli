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
        // 5XgPzWKZSaC8phfRPDG55MMgxaDb35iNRfnPQEbd76nehutdKYU4Stp1ChKZtrjpQYSVZqs9az4p4RootDUwx8Ct cpi_swap_via_stake
        // 5dFfnWJGVga8YnD8DpKcGLjXVyzbC7vwnULxUfrVB1uRauVfxKoEW8AufmezQET2mLvevuZNSz6Nu6o9wdmhQ6yL swap_via_stake
        // 2scJfbaU4VbPDiiLqPB9NGT9U6LWXcxNRW2zw2qRNoKUR2UqmeiYJZ18VzmohBMxtrVwyyd6rGPi6VCrRN2SpFrs stake_wrapped_sol
        // 8KErZjtsQJMnukFxzuEcnqu3cnWyd6nmBFeodXFSGHPWjxsmMB7EBNBs4tvGDakVTqz1CBJukitr1y5NQNK4tq3 cpi_stake_wrapped_sol
        let sig = "8KErZjtsQJMnukFxzuEcnqu3cnWyd6nmBFeodXFSGHPWjxsmMB7EBNBs4tvGDakVTqz1CBJukitr1y5NQNK4tq3";
        let client = RpcClient::new("https://api.mainnet-beta.solana.com");
        let tx = client.get_transaction_with_config(
            &Signature::from_str(sig).unwrap(),
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
