use std::error::Error;

use borsh::BorshDeserialize;
use rusqlite::Connection;
use solana_program::{message::SanitizedMessage, sysvar::instructions::BorrowedInstruction};
use solana_sdk::signature::Signature;
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedConfirmedTransactionWithStatusMeta, UiInstruction,
};
use spl_token::native_mint;
use stakedex_interface::{
    StakeWrappedSolArgs, SwapViaStakeArgs, DEPOSIT_STAKE_IX_DISCM, STAKE_WRAPPED_SOL_IX_DISCM,
    SWAP_VIA_STAKE_IX_DISCM,
};

use crate::subcmd::index::parse::{account_index_of, inner_instructions_of, token_balance_of};

#[derive(Clone, Debug, PartialEq)]
pub struct Invocation {
    pub sig: String,
    pub signer: String,
    pub ix: u8,
    pub unix_timestamp: i64,
    pub slot: u64,
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
        let mut res = top_level_ixs
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
        let inner_ixs = match inner_instructions_of(ectx) {
            Some(slice) => slice,
            None => return res,
        };
        let decoded_inner = inner_ixs.iter().flat_map(|top_level_ix| {
            let top_level_index = top_level_ix.index;
            top_level_ix
                .instructions
                .iter()
                .filter_map(move |inner_ix| {
                    match Self::try_decode_inner_ix(
                        inner_ix,
                        top_level_index,
                        signature,
                        ectx,
                        smsg,
                    ) {
                        Ok(opt) => opt,
                        Err(e) => {
                            println!("WARN: {}", e);
                            None
                        }
                    }
                })
        });
        res.extend(decoded_inner);
        res
    }

    fn try_decode_inner_ix(
        inner_ix: &UiInstruction,
        top_level_index: u8,
        signature: &Signature,
        ectx: &EncodedConfirmedTransactionWithStatusMeta,
        smsg: &SanitizedMessage,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        let inner_ix_compiled = match inner_ix {
            UiInstruction::Compiled(ix) => ix,
            UiInstruction::Parsed(..) => return Err("Not UiInstruction::Compiled".into()),
        };
        let account_keys = smsg.account_keys();
        let program_id = account_keys
            .get(inner_ix_compiled.program_id_index.into())
            .ok_or("No inner program_id")?;
        if *program_id != stakedex_interface::ID {
            return Ok(None);
        }
        let cpi_prog_index = smsg
            .instructions()
            .get(usize::from(top_level_index))
            .ok_or("No top level program index")?
            .program_id_index;
        let cpi_prog = account_keys
            .get(cpi_prog_index.into())
            .ok_or("No top level program")?
            .to_string();
        let tx_meta = ectx
            .transaction
            .meta
            .as_ref()
            .ok_or("No transaction.meta")?;
        let unix_timestamp = ectx.block_time.ok_or("No meta.block_time")?;
        let pre_token_balances = match &tx_meta.pre_token_balances {
            OptionSerializer::Some(v) => v,
            _ => return Err("no pre_token_balances".into()),
        };
        let post_token_balances = match &tx_meta.post_token_balances {
            OptionSerializer::Some(v) => v,
            _ => return Err("no post_token_balances".into()),
        };
        let inner_ix_data = bs58::decode(&inner_ix_compiled.data).into_vec()?;
        let ix = *inner_ix_data.first().ok_or("Empty data")?;
        // luckily, signer is always the first account for all stakedex instructions
        // since payer was removed
        let signer_index = *inner_ix_compiled
            .accounts
            .first()
            .ok_or("No signer index")?;
        let signer = account_keys
            .get(signer_index.into())
            .ok_or("No signer")?
            .to_string();
        let (amount_in, amount_out, mint_in, mint_out) = match ix {
            SWAP_VIA_STAKE_IX_DISCM => {
                let args = SwapViaStakeArgs::deserialize(&mut &inner_ix_data[1..])?;
                let amount_in = args.amount;
                let dest_token_to_index = *inner_ix_compiled
                    .accounts
                    .get(2)
                    .ok_or("SwapViaStake no dest_token_to index")?;
                let dest_token_to_pre = token_balance_of(pre_token_balances, dest_token_to_index)
                    .ok_or("SwapViaStake no dest_token_to pre")?;
                let dest_token_to_post = token_balance_of(post_token_balances, dest_token_to_index)
                    .ok_or("SwapViaStake no dest_token_to post")?;
                let amount_out = dest_token_to_post.saturating_sub(dest_token_to_pre);
                let mint_in_index = *inner_ix_compiled
                    .accounts
                    .get(5)
                    .ok_or("SwapViaStake no mint in index")?;
                let mint_in = account_keys
                    .get(mint_in_index.into())
                    .ok_or("SwapViaStake no mint in")?
                    .to_string();
                let mint_out_index = *inner_ix_compiled
                    .accounts
                    .get(6)
                    .ok_or("SwapViaStake no mint out index")?;
                let mint_out = account_keys
                    .get(mint_out_index.into())
                    .ok_or("SwapViaStake no mint out")?
                    .to_string();
                (amount_in, amount_out, mint_in, mint_out)
            }
            STAKE_WRAPPED_SOL_IX_DISCM => {
                let args = StakeWrappedSolArgs::deserialize(&mut &inner_ix_data[1..])?;
                let amount_in = args.amount;
                let dest_token_to_index = *inner_ix_compiled
                    .accounts
                    .get(2)
                    .ok_or("StakeWrappedSol no dest_token_to index")?;
                let dest_token_to_pre = token_balance_of(pre_token_balances, dest_token_to_index)
                    .ok_or("StakeWrappedSol no dest_token_to pre")?;
                let dest_token_to_post = token_balance_of(post_token_balances, dest_token_to_index)
                    .ok_or("StakeWrappedSol no dest_token_to post")?;
                let amount_out = dest_token_to_post.saturating_sub(dest_token_to_pre);
                let mint_out_index = *inner_ix_compiled
                    .accounts
                    .get(6)
                    .ok_or("StakeWrappedSol no mint out index")?;
                let mint_out = account_keys
                    .get(mint_out_index.into())
                    .ok_or("StakeWrappedSol no mint out")?
                    .to_string();
                (amount_in, amount_out, native_mint::ID.to_string(), mint_out)
            }
            DEPOSIT_STAKE_IX_DISCM => {
                let stake_acc_index = *inner_ix_compiled
                    .accounts
                    .get(1)
                    .ok_or("DepositStake no stake account index")?;
                let amount_in = *tx_meta
                    .pre_balances
                    .get(usize::from(stake_acc_index))
                    .ok_or("DepositStake no stake acc pre_balance")?;
                let dest_token_out_index = *inner_ix_compiled
                    .accounts
                    .get(2)
                    .ok_or("DepositStake no dest_token_out index")?;
                let dest_token_out_pre = token_balance_of(pre_token_balances, dest_token_out_index)
                    .ok_or("DepositStake no dest_token_out pre")?;
                let dest_token_out_post =
                    token_balance_of(post_token_balances, dest_token_out_index)
                        .ok_or("DepositStake no dest_token_out post")?;
                let amount_out = dest_token_out_post.saturating_sub(dest_token_out_pre);
                let mint_out_index = *inner_ix_compiled
                    .accounts
                    .get(4)
                    .ok_or("DepositStake no mint out index")?;
                let mint_out = account_keys
                    .get(usize::from(mint_out_index))
                    .ok_or("DepositStake no mint out")?
                    .to_string();
                (amount_in, amount_out, native_mint::ID.to_string(), mint_out)
            }
            // TODO: admin functions. Skip for now.
            _ => return Ok(None),
        };
        Ok(Some(Self {
            sig: signature.to_string(),
            signer,
            ix,
            unix_timestamp,
            slot: ectx.slot,
            cpi_prog,
            amount_in,
            amount_out,
            mint_in,
            mint_out,
        }))
    }

    // TODO: refactor this to use CompiledInstruction instead of BorrowedInstruction
    // to share more code with try_decode_inner_ix
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
            // TODO: admin functions. Skip for now.
            _ => return Ok(None),
        };
        Ok(Some(Self {
            sig: signature.to_string(),
            signer: signer.pubkey.to_string(),
            ix,
            unix_timestamp,
            slot: ectx.slot,
            cpi_prog: "".into(),
            amount_in,
            amount_out,
            mint_in,
            mint_out,
        }))
    }

    pub fn save(&self, conn: &Connection) -> rusqlite::Result<()> {
        let mut stmt = conn.prepare_cached(
            "INSERT OR REPLACE INTO invocations
            (sig, signer, ix, unix_timestamp, slot, cpi_prog, amount_in, amount_out, mint_in, mint_out)
            VALUES
            (:sig, :signer, :ix, :unix_timestamp, :slot, :cpi_prog, :amount_in, :amount_out, :mint_in, :mint_out)"
        )?;
        stmt.execute(&[
            (":sig", &self.sig),
            (":signer", &self.signer),
            (":ix", &self.ix.to_string()),
            (":unix_timestamp", &self.unix_timestamp.to_string()),
            (":slot", &self.slot.to_string()),
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
    use stakedex_sdk_common::{bsol, cogentsol, jsol};

    use crate::subcmd::index::{db::test_utils::create_test_db, parse::parse_b64_tx};

    use super::*;

    lazy_static! {
        static ref RPC: RpcClient = RpcClient::new("https://api.mainnet-beta.solana.com".into());
    }

    const STAKE_WRAPPED_SOL_AND_SWAP_VIA_STAKE_SIGNER: &str =
        "3etKXcW2fzEJR5YXoSKSmP6UZ633g9uiFv5yuqFUf66k";
    const CPI_SWAP_VIA_STAKE_SIGNER: &str = "FkrJtvLar5nSrNjCC4nsnx1r9BPV18Znuz41P4K4VtTD";
    const CPI_STAKE_WRAPPED_SOL_SIGNER: &str = "4pEhTh7CmBGYAwqExy6YoUFFjdwPXRVNkfkRMt1oCA6G";
    const JUP_PROGRAM_ID: &str = "JUP5cHjnnCx2DppVsufsLrXs8EBZeEZzGtEK9Gdz6ow";

    async fn example_test(json_str: &str, expected: &Invocation) {
        let ectx: EncodedConfirmedTransactionWithStatusMeta =
            serde_json::from_str(json_str).unwrap();
        let (ectx, smsg) = parse_b64_tx(&RPC, ectx).await.unwrap();
        let signature = Signature::from_str(&expected.sig).unwrap();
        let invocations = Invocation::decode(&signature, &ectx, &smsg);
        assert_eq!(invocations.len(), 1);
        let inv = &invocations[0];
        assert_eq!(inv, expected);
    }

    #[test]
    fn test_db_insert() {
        let conn = create_test_db();
        let eg = Invocation {
            sig: "2scJfbaU4VbPDiiLqPB9NGT9U6LWXcxNRW2zw2qRNoKUR2UqmeiYJZ18VzmohBMxtrVwyyd6rGPi6VCrRN2SpFrs".into(),
            signer: STAKE_WRAPPED_SOL_AND_SWAP_VIA_STAKE_SIGNER.into(),
            ix: STAKE_WRAPPED_SOL_IX_DISCM,
            mint_in: native_mint::ID.to_string(),
            mint_out: bsol::ID.to_string(),
            unix_timestamp: 1689827008,
            slot: 206389107,
            cpi_prog: "".into(),
            amount_in: 2_000_000_000,
            amount_out: 1_869_636_257,
        };
        eg.save(&conn).unwrap();
    }

    #[test]
    fn test_db_replace() {
        let conn = create_test_db();
        let eg = Invocation {
            sig: "2scJfbaU4VbPDiiLqPB9NGT9U6LWXcxNRW2zw2qRNoKUR2UqmeiYJZ18VzmohBMxtrVwyyd6rGPi6VCrRN2SpFrs".into(),
            signer: STAKE_WRAPPED_SOL_AND_SWAP_VIA_STAKE_SIGNER.into(),
            ix: STAKE_WRAPPED_SOL_IX_DISCM,
            mint_in: native_mint::ID.to_string(),
            mint_out: bsol::ID.to_string(),
            unix_timestamp: 1689827008,
            slot: 206389107,
            cpi_prog: "".into(),
            amount_in: 2_000_000_000,
            amount_out: 1_869_636_257,
        };
        eg.save(&conn).unwrap();
        eg.save(&conn).unwrap();
    }

    #[tokio::test]
    async fn test_stake_wrapped_sol() {
        let s = include_str!("../../examples/stake_wrapped_sol.json");
        example_test(s, &Invocation {
            sig: "2scJfbaU4VbPDiiLqPB9NGT9U6LWXcxNRW2zw2qRNoKUR2UqmeiYJZ18VzmohBMxtrVwyyd6rGPi6VCrRN2SpFrs".into(),
            signer: STAKE_WRAPPED_SOL_AND_SWAP_VIA_STAKE_SIGNER.into(),
            ix: STAKE_WRAPPED_SOL_IX_DISCM,
            mint_in: native_mint::ID.to_string(),
            mint_out: bsol::ID.to_string(),
            unix_timestamp: 1689827008,
            slot: 206389107,
            cpi_prog: "".into(),
            amount_in: 2_000_000_000,
            amount_out: 1_869_636_257,
        }).await;
    }

    #[tokio::test]
    async fn test_swap_via_stake() {
        let s = include_str!("../../examples/swap_via_stake.json");
        example_test(s, &Invocation {
            sig: "5dFfnWJGVga8YnD8DpKcGLjXVyzbC7vwnULxUfrVB1uRauVfxKoEW8AufmezQET2mLvevuZNSz6Nu6o9wdmhQ6yL".into(),
            signer: STAKE_WRAPPED_SOL_AND_SWAP_VIA_STAKE_SIGNER.into(),
            ix: SWAP_VIA_STAKE_IX_DISCM,
            mint_in: bsol::ID.to_string(),
            mint_out: jsol::ID.to_string(),
            unix_timestamp: 1689827094,
            slot: 206389300,
            cpi_prog: "".into(),
            amount_in: 1_500_000_000,
            amount_out: 1_436_050_745,
        }).await;
    }

    #[tokio::test]
    async fn test_cpi_swap_via_stake() {
        let s = include_str!("../../examples/cpi_swap_via_stake.json");
        example_test(s, &Invocation {
            sig: "5XgPzWKZSaC8phfRPDG55MMgxaDb35iNRfnPQEbd76nehutdKYU4Stp1ChKZtrjpQYSVZqs9az4p4RootDUwx8Ct".into(),
            signer: CPI_SWAP_VIA_STAKE_SIGNER.into(),
            ix: SWAP_VIA_STAKE_IX_DISCM,
            mint_in: jsol::ID.to_string(),
            mint_out: bsol::ID.to_string(),
            unix_timestamp: 1690138252,
            slot: 207087788,
            cpi_prog: JUP_PROGRAM_ID.into(),
            amount_in: 178_228_451,
            amount_out: 0, // because all bsol output was used to swap into other tokens
        }).await;
    }

    #[tokio::test]
    async fn test_cpi_stake_wrapped_sol() {
        let s = include_str!("../../examples/cpi_stake_wrapped_sol.json");
        example_test(s, &Invocation {
            sig: "8KErZjtsQJMnukFxzuEcnqu3cnWyd6nmBFeodXFSGHPWjxsmMB7EBNBs4tvGDakVTqz1CBJukitr1y5NQNK4tq3".into(),
            signer: CPI_STAKE_WRAPPED_SOL_SIGNER.into(),
            ix: STAKE_WRAPPED_SOL_IX_DISCM,
            mint_in: native_mint::ID.to_string(),
            mint_out: cogentsol::ID.to_string(),
            unix_timestamp: 1690292749,
            slot: 207437136,
            cpi_prog: JUP_PROGRAM_ID.into(),
            amount_in: 24_283_800_000,
            amount_out: 26_094_214_514, // because its a jup split route and more cgntSOL was bought from other routes
        }).await;
    }
}
