use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

use crate::bo::event::{PositionMode, Side};

#[derive(Debug, BorshDeserialize)]
pub struct PositionOpened {
    pub id: Pubkey,
    pub feed_id: [u8; 32],
    pub user: Pubkey,
    pub side: Side,
    pub amount: u64,
    pub premium: f64,
    pub premium_amount: u64,
    pub boundary_price: f64,
    pub spot_price: f64,
    pub multiplier: f64,
    pub start_time: i64,
    pub end_time: i64,
    pub mode: PositionMode,
    pub old_sigma2: f64,
    pub new_sigma2: f64,
}
