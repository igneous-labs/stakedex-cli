use clap::Args;
use solana_sdk::{message::Message, system_program, transaction::Transaction};
use stakedex_interface::{record_dex_ix, DexRecord, RecordDexIxArgs, RecordDexKeys};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use crate::{pda::find_dex_record, serde::DexRecordSer, tx_utils::send_or_sim_tx};

use super::SubcmdExec;

const MAX_DEXES_PER_CMD: usize = 6;

#[derive(Args, Debug)]
#[command(long_about = "Save a DexRecord on-chain. Payer must be record authority.")]
pub struct RecordDexArgs {
    #[arg(
        help = "Path(s) to the JSON file of the dex to record. Format is untagged repr or DexRecord, e.g. {\"ty\": \"\", \"mint\": \"...\", \"mainAccount\": \"...\"} . Max 6."
    )]
    pub dexes: Vec<PathBuf>,
}

impl SubcmdExec for RecordDexArgs {
    fn process_cmd(&self, args: &crate::Args) {
        if self.dexes.is_empty() {
            println!("Provide at least 1 dex");
            return;
        }
        if self.dexes.len() > MAX_DEXES_PER_CMD {
            println!("Max {} dexes per cmd", MAX_DEXES_PER_CMD);
            return;
        }

        let client = args.config.rpc_client();
        let payer = args.config.signer();
        let payer_pubkey = payer.pubkey();

        let records = self.dexes.iter().map(|p| load_dex_record_json(p));

        let ixs = records.map(|r| {
            record_dex_ix(
                RecordDexKeys {
                    record_auth: payer_pubkey,
                    payer: payer_pubkey,
                    dex_record: find_dex_record(&r),
                    system_program: system_program::ID,
                },
                RecordDexIxArgs {
                    record_dex_args: stakedex_interface::RecordDexArgs { dex_record: r },
                },
            )
            .unwrap()
        });
        let msg = Message::new(&ixs.collect::<Vec<_>>(), Some(&payer_pubkey));
        let blockhash = client.get_latest_blockhash().unwrap();
        let tx = Transaction::new(&vec![payer], msg, blockhash);

        println!("Recording {} dexes", self.dexes.len(),);
        send_or_sim_tx(args, &client, &tx);
    }
}

fn load_dex_record_json(path: &Path) -> DexRecord {
    let file = File::open(path).unwrap();
    let ser: DexRecordSer = serde_json::from_reader(&file).unwrap();
    ser.into()
}
