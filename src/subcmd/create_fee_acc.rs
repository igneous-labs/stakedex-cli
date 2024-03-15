use clap::Args;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, message::Message, pubkey::Pubkey, system_program,
    transaction::Transaction,
};
use stakedex_interface::{
    create_fee_token_account_ix, CreateFeeTokenAccountIxArgs, CreateFeeTokenAccountKeys,
};
use stakedex_sdk_common::find_fee_token_acc;

use crate::tx_utils::send_or_sim_tx;

use super::SubcmdExec;

#[derive(Args, Debug)]
#[command(
    long_about = "Permissionlessly create the fee token account for a xSOL mint. Required to include it into stakedex."
)]
pub struct CreateFeeAccArgs {
    #[arg(help = "Pubkey of the xSOL mint")]
    pub mint: Pubkey,
}

impl SubcmdExec for CreateFeeAccArgs {
    fn process_cmd(&self, args: &crate::Args) {
        let client = args.config.rpc_client();
        let payer = args.config.signer();

        let fee_token_account = find_fee_token_acc(&self.mint).0;
        let ix = create_fee_token_account_ix(
            CreateFeeTokenAccountKeys {
                payer: payer.pubkey(),
                fee_token_account,
                mint: self.mint,
                token_program: spl_token::ID,
                system_program: system_program::ID,
            },
            CreateFeeTokenAccountIxArgs {},
        )
        .unwrap();
        let msg = Message::new(
            &[
                // TODO: make compute budget dynamic
                ComputeBudgetInstruction::set_compute_unit_limit(20_000),
                ComputeBudgetInstruction::set_compute_unit_price(250),
                ix,
            ],
            Some(&payer.pubkey()),
        );
        let blockhash = client.get_latest_blockhash().unwrap();
        let tx = Transaction::new(&vec![payer], msg, blockhash);

        println!(
            "Creating fee token acc {} of mint {}",
            fee_token_account, self.mint
        );
        send_or_sim_tx(args, &client, &tx);
    }
}
