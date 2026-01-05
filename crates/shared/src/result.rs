use std::{borrow::Cow, num::ParseIntError};

use tracing_error::SpanTrace;

#[derive(Debug, thiserror::Error)]
#[error("{error}\nTrace:\n{trace}")]
pub struct TracedAppErr {
    error: AppErr,
    trace: SpanTrace,
}

impl<E> From<E> for TracedAppErr
where
    E: Into<AppErr>,
{
    fn from(source: E) -> Self {
        TracedAppErr {
            error: source.into(),
            trace: SpanTrace::capture(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppErr {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Custom: {0}")]
    Custom(Cow<'static, str>),

    #[error("ParseIntError: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error(transparent)]
    Database(#[from] sea_orm::error::DbErr),

    #[error(transparent)]
    EvmRpc(#[from] alloy::transports::RpcError<alloy::transports::TransportErrorKind>),

    #[error(transparent)]
    SolTypes(#[from] alloy::sol_types::Error),

    #[error(transparent)]
    WaitReceiptTx(#[from] alloy::providers::PendingTransactionError),
}

pub type Rs<T> = Result<T, TracedAppErr>;

impl AppErr {
    pub fn custom<E: Into<Cow<'static, str>>>(error: E) -> TracedAppErr {
        Self::Custom(error.into()).into()
    }
}
