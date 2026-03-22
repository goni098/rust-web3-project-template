use axum::extract::FromRef;
use database::sea_orm::DatabaseConnection;
use shared::{env::Env, result::Rs};

#[derive(FromRef, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub async fn new() -> Rs<AppState> {
        let db_url = shared::env::read(Env::DatabaseUrl)?;
        let db = database::establish_connection(&db_url).await?;
        Ok(Self { db })
    }
}
