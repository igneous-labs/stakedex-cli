use base64::{engine::general_purpose, Engine};
use bincode::serialize;
use clap::Args;
use solana_program::{program_pack::Pack, system_instruction::withdraw_nonce_account};
use solana_sdk::{message::Message, pubkey::Pubkey, transaction::Transaction};
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
    fn process_cmd(&self, args: &crate::Args) {
        let client = args.config.rpc_client();
        let payer = args.config.signer();

        let mint_token_account = find_fee_token_acc(&self.mint).0;
        let destination_token_account = find_fee_token_acc(
            &self
                .destination
                .unwrap_or_else(|| ADMIN_AUTHORITY.parse().unwrap()),
        )
        .0;

        let account = client.get_account(&mint_token_account).unwrap();
        let parsed = spl_token::state::Account::unpack(&account.data).unwrap();
        let total_fees = parsed.amount;
        let rent_exempt = client
            .get_minimum_balance_for_rent_exemption(account.data.len())
            .unwrap();

        if total_fees < rent_exempt {
            println!("Not enough fees to withdraw");
            return;
        }

        let fees_to_withdraw = total_fees - rent_exempt;

        let ix = withdraw_nonce_account(
            &mint_token_account,
            &payer.pubkey(),
            &destination_token_account,
            fees_to_withdraw,
        );

        let msg = Message::new(&[ix], Some(&payer.pubkey()));
        let blockhash = client.get_latest_blockhash().unwrap();
        let tx = Transaction::new(&vec![payer], msg, blockhash);

        let tx_bytes = serialize(&tx).unwrap();
        let tx_base64 = general_purpose::STANDARD.encode(tx_bytes);

        println!(
            "Transaction for withdrawing fees to {}:\n{:?}",
            destination_token_account, tx_base64
        );
    }
}
