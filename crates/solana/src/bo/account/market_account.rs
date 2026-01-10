use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize, Debug)]
pub struct MarketAccount {
    pub operator: Pubkey,
    pub back_authority: Pubkey,
    pub bump: u8,
    pub min_settle_delay_epochs_secs: u32,
    pub max_settle_delay_epochs_secs: u32,
    pub fee_bps: u16,
    pub min_stake: u64,
    pub max_stake: u64,
    pub min_premium_bps: u16,
    pub call_lambda: f64,
    pub put_lambda: f64,
    pub vega_buffer: f64,
    pub feed_id: [u8; 32],
    pub staleness_max_sec: u32,
    pub price_exponent: i32,
    pub last_price: f64,
    pub last_ts: i64,
    pub sigma2: f64,
    pub min_sigma2: f64,
    pub max_sigma2: f64,
    pub half_life_secs: u64,
    pub vault: Pubkey,
    pub pool: Pubkey,
    pub treasury: Pubkey,
    pub paused: bool,
}
