use std::{path::PathBuf, str::FromStr, time::Duration};

use clap::Args;
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_client::GetConfirmedSignaturesForAddress2Config,
    rpc_config::RpcTransactionConfig, rpc_response::RpcConfirmedTransactionStatusWithSignature,
};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;
use tokio::runtime::Runtime;

use self::{
    consts::{FIRST_NON_ADMIN_SIGNATURE_SINCE_PAYER_REMOVED, MAX_SIGNATURES_FOR_ADDRESS_LIMIT},
    db::{create_conn, earliest_indexed_signature, schema::Invocation},
    parse::parse_b64_tx,
};

use super::SubcmdExec;

mod consts;
mod db;
mod parse;

#[derive(Args, Debug)]
#[command(
    long_about = "Index historical successful stakedex transactions into a sqlite DB, from newest to oldest."
)]
pub struct IndexArgs {
    #[arg(
        help = "Path to sqlite file to save data to",
        default_value = "stakedex.sqlite"
    )]
    pub sqlite_file: PathBuf,
}

impl SubcmdExec for IndexArgs {
    fn process_cmd(&self, args: &crate::Args) {
        let db = create_conn(&self.sqlite_file);
        let mut earliest_sig_opt = earliest_indexed_signature(&db).unwrap();
        let until_sig = Signature::from_str(FIRST_NON_ADMIN_SIGNATURE_SINCE_PAYER_REMOVED).unwrap();
        if let Some(earliest_sig) = earliest_sig_opt.as_ref() {
            if earliest_sig == &until_sig {
                println!("All transactions indexed");
                return;
            }
        }
        let get_transaction_cfg = RpcTransactionConfig {
            encoding: Some(UiTransactionEncoding::Base64),
            commitment: Some(CommitmentConfig::finalized()),
            max_supported_transaction_version: Some(0),
        };
        let rpc = RpcClient::new(args.config.json_rpc_url.clone());
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            // TODO: split this into pipelines:
            // - 1 task for getSignaturesForAddress
            // - multiple tasks for getTransaction
            // - 1 task for writing to DB
            // connected by mpsc channels.
            // Need to check if RPC is ok with this
            loop {
                let get_signatures_res = rpc
                    .get_signatures_for_address_with_config(
                        &stakedex_interface::ID,
                        GetConfirmedSignaturesForAddress2Config {
                            before: earliest_sig_opt,
                            until: Some(until_sig),
                            limit: Some(MAX_SIGNATURES_FOR_ADDRESS_LIMIT),
                            commitment: Some(CommitmentConfig::finalized()),
                        },
                    )
                    .await
                    .unwrap();
                for RpcConfirmedTransactionStatusWithSignature { signature, err, .. } in
                    get_signatures_res
                {
                    let signature = Signature::from_str(&signature).unwrap();
                    earliest_sig_opt.replace(signature);
                    if err.is_none() {
                        let ectx = rpc
                            .get_transaction_with_config(&signature, get_transaction_cfg)
                            .await
                            .unwrap();
                        let (ectx, smsg) = parse_b64_tx(&rpc, ectx).await.unwrap();
                        let invocations = Invocation::decode(&signature, &ectx, &smsg);
                        for invocation in invocations {
                            invocation.save(&db).unwrap();
                        }
                        println!("Indexed {signature}");
                    }
                }
            }
        });
        rt.shutdown_timeout(Duration::from_secs(5));
    }
}
