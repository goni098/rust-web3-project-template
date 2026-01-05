use std::{convert, num::NonZeroUsize, time::Duration};

use alloy::{
    eips::eip1559::Eip1559Estimation,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, TxHash},
    providers::{
        Identity, Provider, ProviderBuilder, RootProvider, WalletProvider,
        fillers::{ChainIdFiller, FillProvider, JoinFill, WalletFiller},
    },
    rpc::{client::RpcClient, types::TransactionRequest},
    signers::local::PrivateKeySigner,
    transports::{http::Http, layers::FallbackLayer},
};
use shared::util::Percent;
use shared::{
    env::Env,
    result::{AppErr, Rs},
};
use tower::ServiceBuilder;
use tracing::instrument;
use url::Url;

pub type PublicClient = FillProvider<JoinFill<Identity, ChainIdFiller>, RootProvider>;
pub type WalletClient = FillProvider<
    JoinFill<JoinFill<Identity, ChainIdFiller>, WalletFiller<EthereumWallet>>,
    RootProvider,
>;

pub fn create_public_client(chain: u64) -> PublicClient {
    let client = creat_root_client(chain);

    ProviderBuilder::new()
        .disable_recommended_fillers()
        .with_chain_id(chain)
        .connect_client(client)
}

pub fn create_wallet_client(chain: u64, signers: Vec<PrivateKeySigner>) -> WalletClient {
    let client = creat_root_client(chain);

    let wallet = signers
        .into_iter()
        .fold(EthereumWallet::default(), |mut wallet, signer| {
            wallet.register_signer(signer);
            wallet
        });

    ProviderBuilder::new()
        .disable_recommended_fillers()
        .with_chain_id(chain)
        .wallet(wallet)
        .connect_client(client)
}

pub trait SendEip1559 {
    fn send_eip1559_tx(
        &self,
        tx: TransactionRequest,
        buffer_gas_in_percent: u8,
        sender: Option<Address>,
    ) -> impl std::future::Future<Output = Rs<TxHash>> + Send;

    fn try_to_send_eip1559_tx(
        &self,
        tx: TransactionRequest,
        sender: Option<Address>,
    ) -> impl std::future::Future<Output = Rs<TxHash>> + Send;
}

impl SendEip1559 for WalletClient {
    #[instrument(skip(tx))]
    async fn send_eip1559_tx(
        &self,
        mut tx: TransactionRequest,
        buffer_gas_in_percent: u8,
        sender: Option<Address>,
    ) -> Rs<TxHash> {
        let sender = sender.unwrap_or_else(|| self.default_signer_address());

        tx.set_from(sender);

        let mut gas_limit = self.estimate_gas(tx.clone()).await?;

        let Eip1559Estimation {
            mut max_fee_per_gas,
            mut max_priority_fee_per_gas,
        } = self.estimate_eip1559_fees().await?;

        max_priority_fee_per_gas += max_priority_fee_per_gas.percent(buffer_gas_in_percent);
        max_fee_per_gas += max_fee_per_gas.percent(buffer_gas_in_percent);
        gas_limit += gas_limit.percent(buffer_gas_in_percent);

        let nonce = self.get_transaction_count(sender).await?;

        tx.set_gas_limit(gas_limit);
        tx.set_nonce(nonce);
        tx.set_max_fee_per_gas(max_fee_per_gas);
        tx.set_max_priority_fee_per_gas(max_priority_fee_per_gas);

        let receipt = self.send_transaction(tx).await?.get_receipt().await?;

        if receipt.status() {
            Ok(receipt.transaction_hash)
        } else {
            Err(AppErr::custom(format!(
                "tx receipt status failed {}",
                receipt.transaction_hash
            )))
        }
    }

    #[instrument(skip(tx))]
    async fn try_to_send_eip1559_tx(
        &self,
        tx: TransactionRequest,
        sender: Option<Address>,
    ) -> Rs<TxHash> {
        let mut attemp_to_send = 0;

        loop {
            let result = tokio::time::timeout(
                Duration::from_secs(90),
                self.send_eip1559_tx(tx.clone(), 12, sender),
            )
            .await
            .map_err(|_| AppErr::custom(format!("Execute tx timeout from chain {:?}", tx.chain_id)))
            .and_then(convert::identity);

            match result {
                Ok(tx_hash) => return Ok(tx_hash),
                Err(error) => {
                    attemp_to_send += 1;

                    if attemp_to_send >= 3 {
                        return Err(error);
                    }

                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
        }
    }
}

fn creat_root_client(chain: u64) -> RpcClient {
    let fallback_layer =
        FallbackLayer::default().with_active_transport_count(NonZeroUsize::new(2).unwrap());

    let (public_rpc, private_rpc) = read_rpcs_by_chain(chain);

    let transports = [Http::new(private_rpc), Http::new(public_rpc)].to_vec();

    let transport = ServiceBuilder::new()
        .layer(fallback_layer)
        .service(transports);

    RpcClient::builder().transport(transport, false)
}

fn read_rpcs_by_chain(chain: u64) -> (Url, Url) {
    let public_rpc = shared::env::read(Env::PubEvmRpc(chain))
        .parse()
        .unwrap_or_else(|_| panic!("invalid public rpc, chain {}", chain));

    let private_rpc = shared::env::read(Env::PriEvmRpc(chain))
        .parse()
        .unwrap_or_else(|_| panic!("invalid private rpc, chain {}", chain));

    (public_rpc, private_rpc)
}
