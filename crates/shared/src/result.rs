use std::{borrow::Cow, num::ParseIntError};

type Location = &'static core::panic::Location<'static>;

#[derive(Debug, thiserror::Error)]
pub enum AppErr {
    #[error("I/O error: {source}\n  at {location}")]
    Io {
        source: std::io::Error,
        location: Location,
    },

    #[error("Custom: {message}")]
    Custom { message: Cow<'static, str> },

    #[error("ParseInt error: {source}\n  at {location}")]
    ParseInt {
        source: ParseIntError,
        location: Location,
    },

    #[error("{source}\n  at {location}")]
    Database {
        source: sea_orm::error::DbErr,
        location: Location,
    },

    #[error("{source}\n  at {location}")]
    EvmRpc {
        source: alloy::transports::RpcError<alloy::transports::TransportErrorKind>,
        location: Location,
    },

    #[error("{source}\n  at {location}")]
    SolTypes {
        source: alloy::sol_types::Error,
        location: Location,
    },

    #[error("{source}\n  at {location}")]
    WaitReceiptTx {
        source: alloy::providers::PendingTransactionError,
        location: Location,
    },

    #[error("{source}\n  at {location}")]
    SolanaClient {
        source: solana_client::client_error::ClientError,
        location: Location,
    },

    #[error("{source}\n  at {location}")]
    ParseSignature {
        source: solana_sdk::signature::ParseSignatureError,
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

pub type Rs<T> = Result<T, AppErr>;

impl AppErr {
    pub fn custom<E: Into<Cow<'static, str>>>(message: E) -> AppErr {
        AppErr::Custom {
            message: message.into(),
        }
    }
}
