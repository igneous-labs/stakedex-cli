//! TODO: this file should be a part of the generated interface/SDK

use solana_sdk::pubkey::Pubkey;

pub const FEE_TOKEN_ACCOUNT_SEED_PREFIX: &[u8; 3] = b"fee";

pub const SOL_BRIDGE_OUT_SEED: &[u8; 14] = b"sol_bridge_out";

pub fn find_fee_token_acc(token_mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[FEE_TOKEN_ACCOUNT_SEED_PREFIX, token_mint.as_ref()],
        &stakedex_interface::ID,
    )
    .0
}

pub fn find_sol_bridge_out() -> Pubkey {
    Pubkey::find_program_address(&[SOL_BRIDGE_OUT_SEED], &stakedex_interface::ID).0
}
