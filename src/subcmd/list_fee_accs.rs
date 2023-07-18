use clap::Args;
use itertools::izip;
use solana_program::{program_pack::Pack, pubkey::Pubkey};
use spl_token::amount_to_ui_amount;
use stakedex_sdk_common::{
    bsol, cogentsol, daosol, esol, find_fee_token_acc, jitosol, jsol, lainesol, msol, risksol,
    scnsol, stsol,
};

use super::SubcmdExec;

#[derive(Args, Debug)]
#[command(long_about = "List all fee token accounts for all xSOL mints")]
pub struct ListFeeAccsArgs;

impl SubcmdExec for ListFeeAccsArgs {
    fn process_cmd(&self, args: &crate::Args) {
        let client = args.config.rpc_client();
        let addrs = ALL_XSOL_MINTS.map(|token_mint| find_fee_token_acc(&token_mint).0);

        let fetched = client.get_multiple_accounts(&addrs).unwrap();
        let iter = izip!(ALL_XSOL_MINTS, addrs, fetched);
        println!("Token | Address | Balance");
        for (mint, addr, opt) in iter {
            let acc = match opt {
                Some(a) => a,
                None => {
                    println!("[WARN] Missing fee acc {} for token {}", addr, mint);
                    continue;
                }
            };
            let parsed = spl_token::state::Account::unpack(&acc.data).unwrap();
            if parsed.mint != mint {
                println!(
                    "[WARN] Wrong mint for acc {}. Expected: {}, got: {}",
                    addr, mint, parsed.mint
                );
                continue;
            }
            // TODO: change decimals if there exists xSOL not of 9 decimals
            println!(
                "{} | {} | {}",
                mint,
                addr,
                amount_to_ui_amount(parsed.amount, spl_token::native_mint::DECIMALS)
            );
        }
    }
}

pub static ALL_XSOL_MINTS: [Pubkey; 12] = [
    bsol::ID,
    cogentsol::ID,
    daosol::ID,
    esol::ID,
    jitosol::ID,
    jsol::ID,
    lainesol::ID,
    msol::ID,
    risksol::ID,
    scnsol::ID,
    spl_token::native_mint::ID,
    stsol::ID,
];
