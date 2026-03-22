use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use shared::{UnionAddress, env::Env};

use crate::{
    exception::{HttpException, HttpResult},
    extractors::auth::Claims,
};

pub fn sign<A>(address: A) -> HttpResult<String>
where
    A: Into<UnionAddress>,
{
    let header = Header::new(Algorithm::HS256);

    let access_secret = shared::env::read(Env::AccessTokenKey)?;

    let now = Utc::now().timestamp();
    let access_exp = now + Duration::days(3).num_seconds();

    let claims = Claims {
        exp: access_exp as u32,
        address: address.into(),
    };

    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_secret(access_secret.as_bytes()),
    )
    .map_err(HttpException::internal)?;

    Ok(token)
}
