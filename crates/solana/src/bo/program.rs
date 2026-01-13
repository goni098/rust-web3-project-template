use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

use crate::bo::event::{PositionOpened, PositionSettled};

pub const PROGRAM_ID: Pubkey = pubkey!("HF1uuMHBmtaCYqhFx2wCFYhkAim8uLaxpMUeWmeTXtD9");

#[derive(Debug)]
#[cmacro::anchor_events(discriminator = 8)]
pub enum BoEvent {
    OpenPosition(PositionOpened),
    SettlePosition(PositionSettled),
}
