use database::repositories;
use database::repositories::settings::Setting;
use database::sea_orm::DatabaseConnection;
use shared::result::Rs;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use tracing::instrument;

use crate::signature::get_the_first_signature;

#[instrument(skip_all)]
pub async fn load_or_init_cursor(db: &DatabaseConnection, client: &RpcClient) -> Rs<Signature> {
    if let Some(sig) = repositories::settings::get(db, Setting::SolCurrentScannedSignature).await? {
        Ok(sig.parse()?)
    } else {
        tracing::info!("Finding the first signature of program...");

        let sig = get_the_first_signature(client)
            .await?
            .expect("not found the first signature");

        repositories::settings::insert(db, Setting::SolCurrentScannedSignature, sig.to_string())
            .await?;

        Ok(sig)
    }
}
