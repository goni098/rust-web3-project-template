mod postion_opened;
mod postion_settled;

use borsh::{BorshDeserialize, BorshSerialize};
pub use postion_opened::*;
pub use postion_settled::*;

#[derive(Debug, BorshDeserialize)]
pub enum PositionResult {
    Won,
    Lose,
    Draw,
    Refund,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum Side {
    Call,
    Put,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum PositionMode {
    Single,
    Batch,
}
