use borsh::BorshDeserialize;
use shared::result::Rs;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use crate::DISCRIMINATOR_SIZE;

/// Fetches and deserializes a Solana account, skipping the 8-byte discriminator
/// 
/// # Arguments
/// * `client` - RPC client for querying Solana
/// * `pubkey` - Public key of the account to fetch
/// 
/// # Returns
/// The deserialized account data
pub async fn fetch_account<T: BorshDeserialize>(client: &RpcClient, pubkey: &Pubkey) -> Rs<T> {
    let data = client.get_account_data(pubkey).await?;
    
    if data.len() < DISCRIMINATOR_SIZE {
        return Err(shared::result::AppErr::custom(
            format!("Account data too short: expected at least {} bytes, got {}", DISCRIMINATOR_SIZE, data.len())
        ));
    }
    
    let buf: &mut &[u8] = &mut &data[DISCRIMINATOR_SIZE..];
    let acc = T::deserialize(buf)?;

    Ok(acc)
}
