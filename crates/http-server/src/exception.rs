use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use std::{borrow::Cow, num::ParseIntError};

use shared::result::AppErr;

type Location = &'static core::panic::Location<'static>;

#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum HttpException {
    #[error("{source}\n  at {location}")]
    Validation {
        source: validator::ValidationErrors,
        location: Location,
    },

    #[error("{source}\n  at {location}")]
    PathRejection {
        source: axum::extract::rejection::PathRejection,
        location: Location,
    },

    #[error("{source}\n  at {location}")]
    FormRejection {
        source: axum::extract::rejection::FormRejection,
        location: Location,
    },

    #[error("{source}\n  at {location}")]
    QueryRejection {
        source: axum::extract::rejection::QueryRejection,
        location: Location,
    },

    #[error("{source}\n  at {location}")]
    BodyRejection {
        source: axum::extract::rejection::JsonRejection,
        location: Location,
    },

    #[error("{message}\n  at {location}")]
    BadRequest {
        message: Cow<'static, str>,
        location: Location,
    },

    #[error("{message}\n  at {location}")]
    Unauthorized {
        message: Cow<'static, str>,
        location: Location,
    },

    #[error("{message}\n  at {location}")]
    Internal {
        message: Cow<'static, str>,
        location: Location,
    },

    #[error("{message}\n  at {location}")]
    Custom {
        message: Cow<'static, str>,
        location: Location,
    },

    #[error("{source}\n  at {location}")]
    ParseInt {
        source: ParseIntError,
        location: Location,
    },

    // AppErr already carries its own location — no need to duplicate
    #[error(transparent)]
    App(#[from] AppErr),
}

pub type HttpResult<A> = Result<A, HttpException>;

macro_rules! impl_from_tracked {
    ($source_type:ty, $variant:ident) => {
        impl From<$source_type> for HttpException {
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

impl_from_tracked!(validator::ValidationErrors, Validation);
impl_from_tracked!(axum::extract::rejection::PathRejection, PathRejection);
impl_from_tracked!(axum::extract::rejection::FormRejection, FormRejection);
impl_from_tracked!(axum::extract::rejection::QueryRejection, QueryRejection);
impl_from_tracked!(axum::extract::rejection::JsonRejection, BodyRejection);
impl_from_tracked!(ParseIntError, ParseInt);

impl HttpException {
    #[track_caller]
    pub fn internal<E: ToString>(error: E) -> Self {
        Self::Internal {
            message: error.to_string().into(),
            location: core::panic::Location::caller(),
        }
    }

    #[track_caller]
    #[allow(dead_code)]
    pub fn bad_request<E: Into<Cow<'static, str>>>(error: E) -> Self {
        Self::BadRequest {
            message: error.into(),
            location: core::panic::Location::caller(),
        }
    }

    #[track_caller]
    pub fn unauthorized<E: Into<Cow<'static, str>>>(error: E) -> Self {
        Self::Unauthorized {
            message: error.into(),
            location: core::panic::Location::caller(),
        }
    }
}

impl IntoResponse for HttpException {
    fn into_response(self) -> axum::response::Response {
        let status_code = match &self {
            Self::BadRequest { .. }
            | Self::PathRejection { .. }
            | Self::FormRejection { .. }
            | Self::QueryRejection { .. }
            | Self::BodyRejection { .. }
            | Self::Validation { .. } => StatusCode::BAD_REQUEST,
            Self::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            _ => {
                tracing::error!("{}", self);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let body = Json(json!({
            "code": status_code.as_u16(),
            "message": self.to_string(),
        }));

        axum::response::IntoResponse::into_response((status_code, body))
    }
}
