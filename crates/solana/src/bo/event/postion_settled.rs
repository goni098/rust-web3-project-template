use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

use crate::bo::event::{PositionResult, Side};

#[derive(Debug, BorshDeserialize)]
pub struct PositionSettled {
    pub id: Pubkey,
    pub user: Pubkey,
    pub side: Side,
    pub boundary_price: f64,
    pub spot_price: f64,
    pub multiplier: f64,
    pub final_price: Option<f64>,
    pub result: PositionResult,
    pub payout: u64,
    pub premium: u64,
}
