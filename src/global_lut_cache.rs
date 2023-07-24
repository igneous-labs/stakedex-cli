use lazy_static::lazy_static;
use solana_address_lookup_table_program::state::AddressLookupTable;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::{collections::HashMap, error::Error};
use tokio::sync::RwLock;

lazy_static! {
    static ref GLOBAL_LUT_CACHE: RwLock<HashMap<Pubkey, Vec<Pubkey>>> = RwLock::new(HashMap::new());
}

pub async fn get_lut(client: &RpcClient, addr: &Pubkey) -> Result<Vec<Pubkey>, Box<dyn Error>> {
    {
        let cache = GLOBAL_LUT_CACHE.read().await;
        if let Some(vec) = cache.get(addr) {
            return Ok(vec.clone());
        }
    }
    let raw = client.get_account_data(addr).await?;
    let AddressLookupTable { addresses, .. } = AddressLookupTable::deserialize(&raw)?;
    {
        let mut cache = GLOBAL_LUT_CACHE.write().await;
        cache.insert(*addr, addresses.clone().into());
    }
    Ok(addresses.into())
}

#[cfg(test)]
mod tests {
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_program::pubkey::Pubkey;
    use std::str::FromStr;

    use super::*;

    #[tokio::test]
    async fn test_get_lut() {
        let client = RpcClient::new("https://solana-mainnet.rpc.extrnode.com".into());
        let fetched = get_lut(
            &client,
            &Pubkey::from_str("51aCqmnbfSuiBt2mF1jvKY5J1AypyLCVNMqa1fiGtZM3").unwrap(),
        )
        .await
        .unwrap();
        assert!(!fetched.is_empty());
    }
}
