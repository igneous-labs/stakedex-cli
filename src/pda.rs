//! TODO: this file should be a part of the generated interface/SDK

use solana_sdk::pubkey::Pubkey;

pub const FEE_TOKEN_ACCOUNT_SEED_PREFIX: &[u8; 3] = b"fee";

pub fn find_fee_token_acc(token_mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[FEE_TOKEN_ACCOUNT_SEED_PREFIX, token_mint.as_ref()],
        &stakedex_interface::ID,
    )
    .0
}
