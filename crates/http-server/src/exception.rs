use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use std::borrow::Cow;

#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum HttpException {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    PathRejection(#[from] axum::extract::rejection::PathRejection),

    #[error(transparent)]
    FormRejection(#[from] axum::extract::rejection::FormRejection),

    #[error(transparent)]
    QueryRejection(#[from] axum::extract::rejection::QueryRejection),

    #[error(transparent)]
    BodyRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error("{0:#?}")]
    BadRequest(Cow<'static, str>),

    #[error("{0:#?}")]
    Unauthorized(Cow<'static, str>),

    #[error("{0:#?}")]
    Internal(Cow<'static, str>),

    #[error("{0:#?}")]
    Custom(Cow<'static, str>),

    #[error(transparent)]
    App(#[from] shared::result::AppErr),

    #[error(transparent)]
    TracedApp(#[from] shared::result::TracedAppErr),
}

pub type HttpResult<A> = Result<A, HttpException>;

impl IntoResponse for HttpException {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::PathRejection(_) => StatusCode::BAD_REQUEST,
            Self::FormRejection(_) => StatusCode::BAD_REQUEST,
            Self::QueryRejection(_) => StatusCode::BAD_REQUEST,
            Self::BodyRejection(_) => StatusCode::BAD_REQUEST,
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            _ => {
                tracing::error!("{:#?} ", self);
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
