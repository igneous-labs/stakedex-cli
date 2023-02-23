//! TODO: this file should be a part of the generated interface/SDK when solores gets PDA derivation capabilities

use borsh::BorshSerialize;
use solana_sdk::pubkey::Pubkey;
use stakedex_interface::{
    DexRecord, DexRecordDepositSol, DexRecordOneWayPoolPair, DexRecordTwoWayPoolPair,
};

pub const DEX_RECORD_SEED_PREFIX: &[u8; 10] = b"dex_record";

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

pub fn find_dex_record(r: &DexRecord) -> Pubkey {
    match r {
        DexRecord::DepositSol(r) => find_dex_record_deposit_sol(r),
        DexRecord::OneWayPoolPair(r) => find_dex_record_one_way_pool_pair(r),
        DexRecord::TwoWayPoolPair(r) => find_dex_record_two_way_pool_pair(r),
    }
}

pub fn find_dex_record_deposit_sol(r: &DexRecordDepositSol) -> Pubkey {
    let mut ty = [0u8];
    r.ty.serialize(&mut ty.as_mut()).unwrap();
    Pubkey::find_program_address(
        &[DEX_RECORD_SEED_PREFIX, &ty, r.mint.as_ref()],
        &stakedex_interface::ID,
    )
    .0
}

pub fn find_dex_record_one_way_pool_pair(r: &DexRecordOneWayPoolPair) -> Pubkey {
    let mut withdraw_stake_ty = [0u8];
    r.withdraw_stake_ty
        .serialize(&mut withdraw_stake_ty.as_mut())
        .unwrap();
    let mut deposit_stake_ty = [0u8];
    r.deposit_stake_ty
        .serialize(&mut deposit_stake_ty.as_mut())
        .unwrap();
    Pubkey::find_program_address(
        &[
            DEX_RECORD_SEED_PREFIX,
            &withdraw_stake_ty,
            &deposit_stake_ty,
            r.withdraw_stake_mint.as_ref(),
            r.deposit_stake_mint.as_ref(),
        ],
        &stakedex_interface::ID,
    )
    .0
}

pub fn find_dex_record_two_way_pool_pair(r: &DexRecordTwoWayPoolPair) -> Pubkey {
    let mut a_ty = [0u8];
    r.a_ty.serialize(&mut a_ty.as_mut()).unwrap();
    let mut b_ty = [0u8];
    r.b_ty.serialize(&mut b_ty.as_mut()).unwrap();
    Pubkey::find_program_address(
        &[
            DEX_RECORD_SEED_PREFIX,
            &a_ty,
            &b_ty,
            r.a_mint.as_ref(),
            r.b_mint.as_ref(),
        ],
        &stakedex_interface::ID,
    )
    .0
}
