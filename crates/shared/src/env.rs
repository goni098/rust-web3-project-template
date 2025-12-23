pub enum Env {
    DatabaseUrl,
    AccessTokenKey,
}

pub fn load() {
    if dotenv::dotenv().is_err() {
        println!("not found .env path, load as default");
    }
}

pub fn read(env: Env) -> String {
    std::env::var(env.key()).unwrap()
}

impl Env {
    fn key(&self) -> &str {
        match self {
            Self::DatabaseUrl => "DATABASE_URL",
            Self::AccessTokenKey => "ACCESS_TOKEN_KEY",
        }
    }
}
