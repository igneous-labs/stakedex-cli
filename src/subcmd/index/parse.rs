use std::error::Error;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::{
    message::{
        v0::{LoadedAddresses, MessageAddressTableLookup},
        AccountKeys, SanitizedMessage, SanitizedVersionedMessage, SimpleAddressLoader,
    },
    pubkey::Pubkey,
};
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, UiTransactionTokenBalance,
};

use crate::global_lut_cache::get_lut;

pub async fn parse_b64_tx(
    client: &RpcClient,
    ectx: EncodedConfirmedTransactionWithStatusMeta,
) -> Result<(EncodedConfirmedTransactionWithStatusMeta, SanitizedMessage), Box<dyn Error>> {
    let versioned_tx = ectx
        .transaction
        .transaction
        .decode()
        .ok_or("could not decode tx".to_owned())?;
    let sanitized = SanitizedVersionedMessage::try_new(versioned_tx.message)?;
    let simple_addr_loader = match sanitized.message.address_table_lookups() {
        None => SimpleAddressLoader::Disabled,
        Some(slice) => {
            let handles: Vec<_> = slice
                .iter()
                .map(
                    |MessageAddressTableLookup {
                         account_key,
                         writable_indexes,
                         readonly_indexes,
                     }| async move {
                        let fetched = get_lut(client, account_key).await?;
                        let [writable_opts, readonly_opts] = [writable_indexes, readonly_indexes]
                            .map(|indexes| {
                                indexes
                                    .iter()
                                    .map(|i| fetched.get::<usize>((*i).into()).copied())
                            });
                        let mut writable = Vec::with_capacity(writable_indexes.len());
                        for opt in writable_opts {
                            match opt {
                                Some(pk) => writable.push(pk),
                                None => return Err("Missing writable LUT pubkey".into()),
                            }
                        }
                        let mut readonly = Vec::with_capacity(readonly_indexes.len());
                        for opt in readonly_opts {
                            match opt {
                                Some(pk) => readonly.push(pk),
                                None => return Err("Missing readonly LUT pubkey".into()),
                            }
                        }
                        Ok::<LoadedAddresses, Box<dyn Error>>(LoadedAddresses {
                            writable,
                            readonly,
                        })
                    },
                )
                .collect();
            let mut writable = Vec::new();
            let mut readonly = Vec::new();
            for handle in handles {
                let loaded = handle.await?;
                writable.extend(loaded.writable);
                readonly.extend(loaded.readonly);
            }
            SimpleAddressLoader::Enabled(LoadedAddresses { writable, readonly })
        }
    };
    Ok((
        ectx,
        SanitizedMessage::try_new(sanitized, simple_addr_loader)?,
    ))
}

pub fn account_index_of(account_keys: &AccountKeys, pk: &Pubkey) -> Option<usize> {
    for (i, maybe_pk) in account_keys.iter().enumerate() {
        if maybe_pk == pk {
            return Some(i);
        }
    }
    None
}

pub fn token_balance_of(v: &[UiTransactionTokenBalance], index: u8) -> Option<u64> {
    for UiTransactionTokenBalance {
        ui_token_amount,
        account_index,
        ..
    } in v
    {
        if index == *account_index {
            return ui_token_amount.amount.parse().ok();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    mod saber_stable_swap_prog {
        use solana_sdk::declare_id;

        declare_id!("SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ");
    }

    #[tokio::test]
    async fn test_parse_b64_tx() {
        let client = RpcClient::new("https://api.mainnet-beta.solana.com".into());
        let s = include_str!("examples/cpi_swap_via_stake.json");
        let ectx = serde_json::from_str(s).unwrap();
        let (_ectx, msg) = parse_b64_tx(&client, ectx).await.unwrap();
        let ixs = msg.decompile_instructions();
        assert_eq!(6, ixs.len());
        // ixs[4] is jup swap
        // accounts[4] should be saber prog
        assert_eq!(saber_stable_swap_prog::ID, *ixs[4].accounts[4].pubkey);
    }
}
