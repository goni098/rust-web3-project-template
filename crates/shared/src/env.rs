use std::borrow::Cow;

pub enum Env {
    DatabaseUrl,
    AccessTokenKey,
    PubEvmRpc(u64),
    PriEvmRpc(u64),
}

pub fn load() {
    if dotenv::dotenv().is_err() {
        println!("not found .env path, load as default");
    }
}

pub fn read(env: Env) -> String {
    std::env::var(env.key().as_ref()).unwrap()
}

impl Env {
    fn key(&self) -> Cow<'static, str> {
        match self {
            Self::DatabaseUrl => "DATABASE_URL".into(),
            Self::AccessTokenKey => "ACCESS_TOKEN_KEY".into(),
            Self::PubEvmRpc(chain) => format!("PUBLIC_RPC_{}", chain).into(),
            Self::PriEvmRpc(chain) => format!("PRIVATE_RPC_{}", chain).into(),
        }
    }
}
