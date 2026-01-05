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
    let var = env.key();
    std::env::var(var.as_ref()).unwrap_or_else(|_| panic!("missing {}", var))
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
