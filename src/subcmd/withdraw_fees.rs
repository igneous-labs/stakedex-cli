use base64::{engine::general_purpose, Engine};
use borsh::ser::BorshSerialize;
use clap::Args;
use solana_sdk::pubkey::Pubkey;
use spl_governance::state::proposal_transaction::{AccountMetaData, InstructionData};
use stakedex_interface::{WithdrawFeesIxArgs, WithdrawFeesKeys};
use stakedex_sdk_common::find_fee_token_acc;

use super::SubcmdExec;

#[derive(Args, Debug)]
#[command(
    long_about = "Outputs a base64-encoded transaction that withdraws fees to the specified token account when signed by the admin authority."
)]
pub struct WithdrawFeesArgs {
    #[arg(help = "Mint of the token to withdraw fees for.")]
    pub mint: Pubkey,
    #[arg(
        help = "Destination token account to withdraw fees to. Defaults to the admin authority's associated token account if not provided."
    )]
    pub destination: Option<Pubkey>,
}

const ADMIN_AUTHORITY: &str = "A7jn1BA6LPHX8Wcmc8t476gjoLCG4PZakww19ZXfFRjX";

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

        let ix_data = InstructionData {
            program_id: spl_token::ID,
            accounts: ix
                .accounts
                .iter()
                .map(|acc| AccountMetaData {
                    pubkey: acc.pubkey,
                    is_signer: acc.is_signer,
                    is_writable: acc.is_writable,
                })
                .collect(),
            data: ix.data,
        };

        let serialized_data: Vec<u8> = ix_data.try_to_vec().unwrap();
        let ix_base64 = general_purpose::STANDARD.encode(serialized_data);

        println!(
            "Instruction for withdrawing fees to {}:\n{}",
            destination_token_account, ix_base64
        );
    }
}
