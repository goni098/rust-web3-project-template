use borsh::BorshDeserialize;
use shared::result::Rs;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use crate::DISCRIMINATOR_SIZE;

pub async fn fetch_account<T: BorshDeserialize>(client: &RpcClient, pubkey: &Pubkey) -> Rs<T> {
    let data = client.get_account_data(pubkey).await?;
    let buf: &mut &[u8] = &mut &data.as_slice()[DISCRIMINATOR_SIZE..];
    let acc = T::deserialize(buf)?;

    Ok(acc)
}
