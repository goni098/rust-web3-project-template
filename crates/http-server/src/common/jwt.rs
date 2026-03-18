use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use shared::{UnionAddress, env::Env};

use crate::{
    exception::{HttpException, HttpResult},
    extractors::auth::Claims,
};

pub fn sign(address: UnionAddress) -> HttpResult<String> {
    let header = Header::new(Algorithm::HS256);

    let access_secret = shared::env::read(Env::AccessTokenKey);

    let now = Utc::now().timestamp();
    let access_exp = now + Duration::days(3).num_seconds();

    let claims = Claims {
        exp: access_exp as u32,
        address,
    };

    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_secret(access_secret.as_bytes()),
    )
    .map_err(HttpException::internal)?;

    Ok(token)
}
