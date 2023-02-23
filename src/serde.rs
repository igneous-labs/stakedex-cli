//! serde formats and struct wrappers.
//! A lot of copy-pasting because solores types dont impl serde

use std::mem::swap;

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use solana_sdk::pubkey::Pubkey;
use stakedex_interface::{
    DepositSolType, DepositStakeType, DepositWithdrawStakeType, DexRecord, DexRecordDepositSol,
    DexRecordOneWayPoolPair, DexRecordTwoWayPoolPair, WithdrawStakeType,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DexRecordSer {
    DepositSol(DexRecordDepositSolSer),
    OneWayPoolPair(DexRecordOneWayPoolPairSer),
    TwoWayPoolPair(DexRecordTwoWayPoolPairSer),
}

impl From<DexRecordSer> for DexRecord {
    fn from(a: DexRecordSer) -> Self {
        match a {
            DexRecordSer::DepositSol(s) => Self::DepositSol(s.into()),
            DexRecordSer::OneWayPoolPair(s) => Self::OneWayPoolPair(s.into()),
            DexRecordSer::TwoWayPoolPair(s) => Self::TwoWayPoolPair(s.into()),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DexRecordDepositSolSer {
    pub ty: DepositSolTypeInt,
    pub mint: B58Pubkey,
    pub main_account: B58Pubkey,
}

impl From<DexRecordDepositSolSer> for DexRecordDepositSol {
    fn from(a: DexRecordDepositSolSer) -> Self {
        Self {
            ty: a.ty.into(),
            mint: a.mint.0,
            main_account: a.main_account.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DexRecordOneWayPoolPairSer {
    pub withdraw_stake_ty: WithdrawStakeTypeInt,
    pub deposit_stake_ty: DepositStakeTypeInt,
    pub withdraw_stake_mint: B58Pubkey,
    pub deposit_stake_mint: B58Pubkey,
    pub withdraw_stake_main_account: B58Pubkey,
    pub deposit_stake_main_account: B58Pubkey,
}

impl From<DexRecordOneWayPoolPairSer> for DexRecordOneWayPoolPair {
    fn from(a: DexRecordOneWayPoolPairSer) -> Self {
        Self {
            withdraw_stake_ty: a.withdraw_stake_ty.into(),
            deposit_stake_ty: a.deposit_stake_ty.into(),
            withdraw_stake_mint: a.withdraw_stake_mint.0,
            deposit_stake_mint: a.deposit_stake_mint.0,
            withdraw_stake_main_account: a.withdraw_stake_main_account.0,
            deposit_stake_main_account: a.deposit_stake_main_account.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DexRecordTwoWayPoolPairSer {
    pub a_ty: DepositWithdrawStakeTypeInt,
    pub b_ty: DepositWithdrawStakeTypeInt,
    pub a_mint: B58Pubkey,
    pub b_mint: B58Pubkey,
    pub a_main_account: B58Pubkey,
    pub b_main_account: B58Pubkey,
}

impl From<DexRecordTwoWayPoolPairSer> for DexRecordTwoWayPoolPair {
    fn from(a: DexRecordTwoWayPoolPairSer) -> Self {
        let mut res = Self {
            a_ty: a.a_ty.into(),
            b_ty: a.b_ty.into(),
            a_mint: a.a_mint.0,
            b_mint: a.b_mint.0,
            a_main_account: a.a_main_account.0,
            b_main_account: a.b_main_account.0,
        };
        if res.a_mint > res.b_mint {
            swap(&mut res.a_ty, &mut res.b_ty);
            swap(&mut res.a_mint, &mut res.b_mint);
            swap(&mut res.a_main_account, &mut res.b_main_account);
        }
        res
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DepositSolTypeInt {
    Eversol,
    Lido,
    Marinade,
    Socean,
    Spl,
}

impl From<DepositSolTypeInt> for DepositSolType {
    fn from(a: DepositSolTypeInt) -> Self {
        match a {
            DepositSolTypeInt::Eversol => Self::Eversol,
            DepositSolTypeInt::Lido => Self::Lido,
            DepositSolTypeInt::Marinade => Self::Marinade,
            DepositSolTypeInt::Socean => Self::Socean,
            DepositSolTypeInt::Spl => Self::Spl,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DepositStakeTypeInt {
    Eversol,
    Marinade,
    Socean,
    Spl,
    Unstakeit,
}

impl From<DepositStakeTypeInt> for DepositStakeType {
    fn from(a: DepositStakeTypeInt) -> Self {
        match a {
            DepositStakeTypeInt::Eversol => Self::Eversol,
            DepositStakeTypeInt::Marinade => Self::Marinade,
            DepositStakeTypeInt::Socean => Self::Socean,
            DepositStakeTypeInt::Spl => Self::Spl,
            DepositStakeTypeInt::Unstakeit => Self::Unstakeit,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum WithdrawStakeTypeInt {
    Eversol,
    Lido,
    Socean,
    Spl,
}

impl From<WithdrawStakeTypeInt> for WithdrawStakeType {
    fn from(a: WithdrawStakeTypeInt) -> Self {
        match a {
            WithdrawStakeTypeInt::Eversol => Self::Eversol,
            WithdrawStakeTypeInt::Lido => Self::Lido,
            WithdrawStakeTypeInt::Socean => Self::Socean,
            WithdrawStakeTypeInt::Spl => Self::Spl,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DepositWithdrawStakeTypeInt {
    Eversol,
    Socean,
    Spl,
}

impl From<DepositWithdrawStakeTypeInt> for DepositWithdrawStakeType {
    fn from(a: DepositWithdrawStakeTypeInt) -> Self {
        match a {
            DepositWithdrawStakeTypeInt::Eversol => Self::Eversol,
            DepositWithdrawStakeTypeInt::Socean => Self::Socean,
            DepositWithdrawStakeTypeInt::Spl => Self::Spl,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct B58Pubkey(pub Pubkey);

struct B58PubkeyVistor;

impl<'de> Visitor<'de> for B58PubkeyVistor {
    type Value = B58Pubkey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("base-58 encoded string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bytes = bs58::decode(value)
            .into_vec()
            .map_err(|e| de::Error::custom(&format!("invalid base-58 string. Error: {:?}", e)))?;
        let bytes_arr = <[u8; 32]>::try_from(<&[u8]>::clone(&&bytes[..]))
            .map_err(|e| de::Error::custom(&format!("Not 256-bit long. Error: {:?}", e)))?;
        Ok(B58Pubkey(Pubkey::new_from_array(bytes_arr)))
    }
}

impl<'de> Deserialize<'de> for B58Pubkey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(B58PubkeyVistor)
    }
}

impl Serialize for B58Pubkey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&bs58::encode(self.0).into_string())
    }
}
