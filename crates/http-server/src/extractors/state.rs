use axum::extract::FromRef;
use database::sea_orm::DatabaseConnection;
use shared::env::Env;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub secrets: Secrets,
}

#[derive(Clone)]
pub struct Secrets {
    pub access_token_key: String,
}

impl AppState {
    pub async fn new() -> AppState {
        let access_token_key = shared::env::read(Env::AccessTokenKey);
        let db_url = shared::env::read(Env::DatabaseUrl);

        let db = database::establish_connection(&db_url)
            .await
            .expect("can not connect to database");

        let secrets = Secrets { access_token_key };

        Self { db, secrets }
    }
}
