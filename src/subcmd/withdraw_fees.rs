use clap::Args;
use solana_sdk::{pubkey::Pubkey, transaction::Transaction};
use stakedex_interface::{WithdrawFeesIxArgs, WithdrawFeesKeys};
use stakedex_sdk_common::find_fee_token_acc;

use super::SubcmdExec;

#[derive(Args, Debug)]
#[command(
    long_about = "Outputs a base64-encoded spl-governance program instruction that withdraws fees to the specified token account when executed by the admin authority spl-governance PDA."
)]
pub struct WithdrawFeesArgs {
    #[arg(help = "Mint of the token to withdraw fees for.")]
    pub mint: Pubkey,
    #[arg(
        help = "Destination token account to withdraw fees to. Defaults to the admin authority's associated token account if not provided."
    )]
    pub destination: Option<Pubkey>,
}
/// TODO: import from stakedex crate when onchain program is made public
const ADMIN_AUTHORITY: &str = "EeQmNqm1RcQnee8LTyx6ccVG9FnR8TezQuw2JXq2LC1T";

impl SubcmdExec for WithdrawFeesArgs {
    fn process_cmd(&self, _args: &crate::Args) {
        let mint_token_account = find_fee_token_acc(&self.mint).0;
        let admin = ADMIN_AUTHORITY.parse().unwrap();
        let destination_token_account = self.destination.unwrap_or_else(|| {
            spl_associated_token_account::get_associated_token_address(&admin, &self.mint)
        });

        let ix = stakedex_interface::withdraw_fees_ix(
            WithdrawFeesKeys {
                admin,
                mint: self.mint,
                fee_token_account: mint_token_account,
                withdraw_to: destination_token_account,
                token_program: spl_token::ID,
            },
            WithdrawFeesIxArgs {},
        )
        .unwrap();

        let tx = Transaction::new_with_payer(&[ix], None);

        let serialized_data: Vec<u8> = bincode::serialize(&tx).unwrap();
        let ix_encoded = bs58::encode(serialized_data).into_string();

        println!(
            "Tx for withdrawing fees to {}:\n{}",
            destination_token_account, ix_encoded
        );
    }
}
