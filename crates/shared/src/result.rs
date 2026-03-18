use std::{borrow::Cow, num::ParseIntError};

type Location = &'static core::panic::Location<'static>;

#[derive(Debug, thiserror::Error)]
pub enum AppErr {
    #[error("I/O: {source}")]
    Io {
        source: std::io::Error,
        location: Location,
    },

    #[error("Custom: {message}")]
    Custom {
        message: Cow<'static, str>,
        location: Location,
    },

    #[error("ParseInt: {source}")]
    ParseInt {
        source: ParseIntError,
        location: Location,
    },

    #[error("Database: {source}")]
    Database {
        source: sea_orm::error::DbErr,
        location: Location,
    },

    #[error("EvmRpc: {source}")]
    EvmRpc {
        source: alloy::transports::RpcError<alloy::transports::TransportErrorKind>,
        location: Location,
    },

    #[error("SolTypes: {source}")]
    SolTypes {
        source: alloy::sol_types::Error,
        location: Location,
    },

    #[error("WaitReceiptTx: {source}")]
    WaitReceiptTx {
        source: alloy::providers::PendingTransactionError,
        location: Location,
    },

    #[error("SolanaClient: {source}")]
    SolanaClient {
        source: solana_client::client_error::ClientError,
        location: Location,
    },

    #[error("ParseSignature: {source}")]
    ParseSignature {
        source: solana_sdk::signature::ParseSignatureError,
        location: Location,
    },

    #[error("ParseHexAddress: {source}")]
    ParseHexAddress {
        source: alloy::hex::FromHexError,
        location: Location,
    },

    #[error("ParseSolanaPubkey: {source}")]
    ParseSolanaPubkey {
        source: solana_sdk::pubkey::ParsePubkeyError,
        location: Location,
    },
}

macro_rules! impl_from_tracked {
    ($source_type:ty, $variant:ident) => {
        impl From<$source_type> for AppErr {
            #[track_caller]
            fn from(source: $source_type) -> Self {
                Self::$variant {
                    source,
                    location: core::panic::Location::caller(),
                }
            }
        }
    };
}

impl_from_tracked!(std::io::Error, Io);
impl_from_tracked!(ParseIntError, ParseInt);
impl_from_tracked!(sea_orm::error::DbErr, Database);
impl_from_tracked!(
    alloy::transports::RpcError<alloy::transports::TransportErrorKind>,
    EvmRpc
);
impl_from_tracked!(alloy::sol_types::Error, SolTypes);
impl_from_tracked!(alloy::providers::PendingTransactionError, WaitReceiptTx);
impl_from_tracked!(solana_client::client_error::ClientError, SolanaClient);
impl_from_tracked!(solana_sdk::signature::ParseSignatureError, ParseSignature);
impl_from_tracked!(solana_sdk::pubkey::ParsePubkeyError, ParseSolanaPubkey);
impl_from_tracked!(alloy::hex::FromHexError, ParseHexAddress);

pub type Rs<T> = Result<T, AppErr>;

impl AppErr {
    pub fn location(&self) -> Location {
        match self {
            AppErr::Custom { location, .. } => location,
            AppErr::Database { location, .. } => location,
            AppErr::EvmRpc { location, .. } => location,
            AppErr::Io { location, .. } => location,
            AppErr::ParseInt { location, .. } => location,
            AppErr::ParseSignature { location, .. } => location,
            AppErr::SolTypes { location, .. } => location,
            AppErr::SolanaClient { location, .. } => location,
            AppErr::WaitReceiptTx { location, .. } => location,
            AppErr::ParseHexAddress { location, .. } => location,
            AppErr::ParseSolanaPubkey { location, .. } => location,
        }
    }

    pub fn trace<C: AsRef<str>>(&self, ctx: C) {
        let location = self.location();
        tracing::error!("{} >> {}\nTrace: {}", ctx.as_ref(), self, location);
    }

    #[track_caller]
    pub fn custom<E: Into<Cow<'static, str>>>(message: E) -> AppErr {
        AppErr::Custom {
            message: message.into(),
            location: core::panic::Location::caller(),
        }
    }
}
