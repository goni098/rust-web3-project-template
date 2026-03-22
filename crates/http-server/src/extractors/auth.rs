use crate::exception::{HttpException, HttpResult};
use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, errors::ErrorKind};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use shared::{UnionAddress, env::Env};

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub exp: u32,
    pub address: UnionAddress,
}

pub struct Auth(pub Claims);

impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = HttpException;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> HttpResult<Self> {
        let secret = shared::env::read(Env::AccessTokenKey)?;

        parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| HttpException::unauthorized("Missing Authorization"))
            .and_then(|bearer| decode_token(bearer.token(), &secret))
            .map(Self)
    }
}

fn decode_token<T: DeserializeOwned>(token: &str, secret: &str) -> HttpResult<T> {
    jsonwebtoken::decode::<T>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|err| match err.kind() {
        ErrorKind::ExpiredSignature => HttpException::unauthorized("Expired token"),
        _ => HttpException::unauthorized("Invalid token"),
    })
    .map(|token_data| token_data.claims)
}
