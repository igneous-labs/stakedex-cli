use clap::Args;
use solana_sdk::{
    message::Message, program_pack::Pack, system_instruction, transaction::Transaction,
};
use spl_token::state::Account;
use stakedex_sdk_common::find_sol_bridge_out;

use crate::tx_utils::send_or_sim_tx;

use super::SubcmdExec;

#[derive(Args, Debug)]
#[command(
    long_about = "Permissionlessly fund the sol_bridge_out account with spl token account rent exempt lamports. Required for stake wrapped SOL instruction to work."
)]
pub struct FundSolBridgeArgs;

impl SubcmdExec for FundSolBridgeArgs {
    fn process_cmd(&self, args: &crate::Args) {
        let client = args.config.rpc_client();
        let payer = args.config.signer();

        let sol_bridge_out = find_sol_bridge_out().0;

        let existing = client.get_balance(&sol_bridge_out).unwrap();
        let lamports_req = client
            .get_minimum_balance_for_rent_exemption(Account::get_packed_len())
            .unwrap();

        if existing >= lamports_req {
            println!(
                "{} already has enough ({} lamports >= {} required)",
                sol_bridge_out, existing, lamports_req
            );
            return;
        }

        let ix = system_instruction::transfer(&payer.pubkey(), &sol_bridge_out, lamports_req);
        let msg = Message::new(&[ix], Some(&payer.pubkey()));
        let blockhash = client.get_latest_blockhash().unwrap();
        let tx = Transaction::new(&vec![payer], msg, blockhash);

        println!(
            "Transfering {} lamports to {}",
            lamports_req, sol_bridge_out,
        );
        send_or_sim_tx(args, &client, &tx);
    }
}
