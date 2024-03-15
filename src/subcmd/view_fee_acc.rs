use clap::Args;
use solana_sdk::{native_token::lamports_to_sol, program_pack::Pack, pubkey::Pubkey};
use stakedex_sdk_common::find_fee_token_acc;

use super::SubcmdExec;

#[derive(Args, Debug)]
#[command(long_about = "View the fee token account for a xSOL mint")]
pub struct ViewFeeAccArgs {
    #[arg(help = "Pubkey of the xSOL mint")]
    pub mint: Pubkey,
}

impl SubcmdExec for ViewFeeAccArgs {
    fn process_cmd(&self, args: &crate::Args) {
        let client = args.config.rpc_client();
        let fee_token_account = find_fee_token_acc(&self.mint).0;
        let fetched = client.get_account(&fee_token_account).unwrap();
        let parsed = spl_token::state::Account::unpack(&fetched.data).unwrap();
        println!("Account: {fee_token_account}");
        println!("Balance: {}", lamports_to_sol(parsed.amount));
    }
}
