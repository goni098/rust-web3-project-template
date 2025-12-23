use crate::{
    exception::{
        HttpException::{self, *},
        HttpResult,
    },
    extractors::state::Secrets,
};
use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, errors::ErrorKind};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub exp: u32,
    pub id: i64,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Sub {
    pub exp: u32,
    pub sub: i64,
}

pub struct Auth(pub Claims);

impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
    Secrets: FromRef<S>,
{
    type Rejection = HttpException;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> HttpResult<Self> {
        let access_token_secret = Secrets::from_ref(state).access_token_key;

        parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Unauthorized("Missing Authorization".into()))
            .and_then(|bearer| decode_token(bearer.token(), &access_token_secret))
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
        ErrorKind::ExpiredSignature => Unauthorized("Expired token".into()),
        _ => Unauthorized("Invalid token".into()),
    })
    .map(|token_data| token_data.claims)
}
