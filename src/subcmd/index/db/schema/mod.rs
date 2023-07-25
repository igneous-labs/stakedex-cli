use std::error::Error;

use borsh::BorshDeserialize;
use rusqlite::Connection;
use solana_program::{message::SanitizedMessage, sysvar::instructions::BorrowedInstruction};
use solana_sdk::signature::Signature;
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedConfirmedTransactionWithStatusMeta,
};
use spl_token::native_mint;
use stakedex_interface::{
    StakeWrappedSolArgs, SwapViaStakeArgs, DEPOSIT_STAKE_IX_DISCM, STAKE_WRAPPED_SOL_IX_DISCM,
    SWAP_VIA_STAKE_IX_DISCM,
};

use crate::subcmd::index::parse::{account_index_of, token_balance_of};

pub struct Invocation {
    pub sig: String,
    pub signer: String,
    pub ix: u8,
    pub unix_timestamp: i64,
    pub cpi_prog: String,
    pub amount_in: u64,
    pub amount_out: u64,
    pub mint_in: String,
    pub mint_out: String,
}

impl Invocation {
    pub fn decode(
        signature: &Signature,
        ectx: &EncodedConfirmedTransactionWithStatusMeta,
        smsg: &SanitizedMessage,
    ) -> Vec<Self> {
        let top_level_ixs = smsg.decompile_instructions();
        let res = top_level_ixs
            .iter()
            .filter_map(|top_ix| {
                match Self::try_decode_top_level_ix(top_ix, signature, ectx, smsg) {
                    Ok(opt) => opt,
                    Err(e) => {
                        println!("WARN: {}", e);
                        None
                    }
                }
            })
            .collect();
        res
    }

    fn try_decode_top_level_ix(
        top_ix: &BorrowedInstruction,
        signature: &Signature,
        ectx: &EncodedConfirmedTransactionWithStatusMeta,
        smsg: &SanitizedMessage,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        if *top_ix.program_id != stakedex_interface::ID {
            return Ok(None);
        }
        let account_keys = smsg.account_keys();
        let tx_meta = ectx
            .transaction
            .meta
            .as_ref()
            .ok_or("No transaction.meta")?;
        let unix_timestamp = ectx.block_time.ok_or("No meta.block_time")?;
        let ix = *top_ix.data.first().ok_or("Empty data")?;
        let pre_token_balances = match &tx_meta.pre_token_balances {
            OptionSerializer::Some(v) => v,
            _ => return Err("no pre_token_balances".into()),
        };
        let post_token_balances = match &tx_meta.post_token_balances {
            OptionSerializer::Some(v) => v,
            _ => return Err("no post_token_balances".into()),
        };
        // luckily, signer is always the first account for all stakedex instructions
        // since payer was removed
        let accounts = &top_ix.accounts;
        let signer = accounts.first().ok_or("No signer")?;
        let (amount_in, amount_out, mint_in, mint_out) = match ix {
            SWAP_VIA_STAKE_IX_DISCM => {
                let args = SwapViaStakeArgs::deserialize(&mut &top_ix.data[1..])?;
                let amount_in = args.amount;
                let dest_token_to = accounts
                    .get(2)
                    .ok_or("SwapViaStake no dest_token_to")?
                    .pubkey;
                let dest_token_to_index: u8 = account_index_of(&account_keys, dest_token_to)
                    .ok_or("SwapViaStake no dest_token_to index")?
                    .try_into()
                    .unwrap();
                let dest_token_to_pre = token_balance_of(pre_token_balances, dest_token_to_index)
                    .ok_or("SwapViaStake no dest_token_to pre")?;
                let dest_token_to_post = token_balance_of(post_token_balances, dest_token_to_index)
                    .ok_or("SwapViaStake no dest_token_to post")?;
                let amount_out = dest_token_to_post.saturating_sub(dest_token_to_pre);
                let mint_in = accounts
                    .get(5)
                    .ok_or("SwapViaStake no mint in")?
                    .pubkey
                    .to_string();
                let mint_out = accounts
                    .get(6)
                    .ok_or("SwapViaStake no mint out")?
                    .pubkey
                    .to_string();
                (amount_in, amount_out, mint_in, mint_out)
            }
            STAKE_WRAPPED_SOL_IX_DISCM => {
                let args = StakeWrappedSolArgs::deserialize(&mut &top_ix.data[1..])?;
                let amount_in = args.amount;
                let dest_token_to = accounts
                    .get(2)
                    .ok_or("StakeWrappedSol no dest_token_to")?
                    .pubkey;
                let dest_token_to_index: u8 = account_index_of(&account_keys, dest_token_to)
                    .ok_or("StakeWrappedSol no dest_token_to index")?
                    .try_into()
                    .unwrap();
                let dest_token_to_pre = token_balance_of(pre_token_balances, dest_token_to_index)
                    .ok_or("StakeWrappedSol no dest_token_to pre")?;
                let dest_token_to_post = token_balance_of(post_token_balances, dest_token_to_index)
                    .ok_or("StakeWrappedSol no dest_token_to post")?;
                let amount_out = dest_token_to_post.saturating_sub(dest_token_to_pre);
                let mint_out = accounts
                    .get(6)
                    .ok_or("StakeWrappedSol no mint out")?
                    .pubkey
                    .to_string();
                (amount_in, amount_out, native_mint::ID.to_string(), mint_out)
            }
            DEPOSIT_STAKE_IX_DISCM => {
                let stake_acc = accounts
                    .get(1)
                    .ok_or("DepositStake no stake account")?
                    .pubkey;
                let stake_acc_index = account_index_of(&account_keys, stake_acc)
                    .ok_or("DepositStake no stake account index")?;
                let amount_in = *tx_meta
                    .pre_balances
                    .get(stake_acc_index)
                    .ok_or("DepositStake no stake acc pre_balance")?;
                let dest_token_out = accounts
                    .get(2)
                    .ok_or("DepositStake no dest token out")?
                    .pubkey;
                let dest_token_out_index: u8 = account_index_of(&account_keys, dest_token_out)
                    .ok_or("DepositStake no dest_token_out index")?
                    .try_into()
                    .unwrap();
                let dest_token_out_pre = token_balance_of(pre_token_balances, dest_token_out_index)
                    .ok_or("DepositStake no dest_token_out pre")?;
                let dest_token_out_post =
                    token_balance_of(post_token_balances, dest_token_out_index)
                        .ok_or("DepositStake no dest_token_out post")?;
                let amount_out = dest_token_out_post.saturating_sub(dest_token_out_pre);
                let mint_out = accounts
                    .get(4)
                    .ok_or("DepositStake no mint out")?
                    .pubkey
                    .to_string();
                (amount_in, amount_out, native_mint::ID.to_string(), mint_out)
            }
            // TODO: admin functions
            _ => return Ok(None),
        };
        Ok(Some(Self {
            sig: signature.to_string(),
            signer: signer.pubkey.to_string(),
            ix,
            unix_timestamp,
            cpi_prog: "".into(),
            amount_in,
            amount_out,
            mint_in,
            mint_out,
        }))
    }

    pub fn save(&self, conn: &Connection) -> rusqlite::Result<()> {
        let mut stmt = conn.prepare_cached(
            "INSERT INTO invocations (sig, signer, ix, unix_timestamp, cpi_prog, amount_in, amount_out, mint_in, mint_out) VALUES (:sig, :signer, :ix, :unix_timestamp, :cpi_prog, :amount_in, :amount_out, :mint_in, :mint_out)"
        )?;
        stmt.execute(&[
            (":sig", &self.sig),
            (":signer", &self.signer),
            (":ix", &self.ix.to_string()),
            (":unix_timestamp", &self.unix_timestamp.to_string()),
            (":cpi_prog", &self.cpi_prog),
            (":amount_in", &self.amount_in.to_string()),
            (":amount_out", &self.amount_out.to_string()),
            (":mint_in", &self.mint_in.to_string()),
            (":mint_out", &self.mint_out.to_string()),
        ])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use lazy_static::lazy_static;
    use solana_client::nonblocking::rpc_client::RpcClient;
    use stakedex_sdk_common::bsol;

    use crate::subcmd::index::parse::parse_b64_tx;

    use super::*;

    lazy_static! {
        static ref RPC: RpcClient = RpcClient::new("https://api.mainnet-beta.solana.com".into());
    }

    const STAKE_WRAPPED_SOL_SIGNER: &str = "3etKXcW2fzEJR5YXoSKSmP6UZ633g9uiFv5yuqFUf66k";

    #[tokio::test]
    async fn test_stake_wrapped_sol() {
        let s = include_str!("../../examples/stake_wrapped_sol.json");
        let ectx: EncodedConfirmedTransactionWithStatusMeta = serde_json::from_str(s).unwrap();
        let (ectx, smsg) = parse_b64_tx(&RPC, ectx).await.unwrap();
        let sig_str = "2scJfbaU4VbPDiiLqPB9NGT9U6LWXcxNRW2zw2qRNoKUR2UqmeiYJZ18VzmohBMxtrVwyyd6rGPi6VCrRN2SpFrs";
        let signature = Signature::from_str(sig_str).unwrap();
        let invocations = Invocation::decode(&signature, &ectx, &smsg);
        assert_eq!(invocations.len(), 1);
        let inv = &invocations[0];

        assert_eq!(inv.sig, sig_str);
        assert_eq!(inv.amount_in, 2_000_000_000);
        assert_eq!(inv.amount_out, 1_869_636_257);
        assert_eq!(inv.cpi_prog, "");
        assert_eq!(inv.ix, STAKE_WRAPPED_SOL_IX_DISCM);
        assert_eq!(inv.mint_in, native_mint::ID.to_string());
        assert_eq!(inv.mint_out, bsol::ID.to_string());
        assert_eq!(inv.unix_timestamp, 1689827008);
        assert_eq!(inv.signer, STAKE_WRAPPED_SOL_SIGNER);
    }
}
