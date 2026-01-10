use base64::{Engine, prelude::BASE64_STANDARD};
use borsh::BorshDeserialize;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

use crate::{
    DISCRIMINATOR_SIZE,
    bo::event::{PositionOpened, PositionSettled},
};

pub const PROGRAM_ID: Pubkey = pubkey!("HF1uuMHBmtaCYqhFx2wCFYhkAim8uLaxpMUeWmeTXtD9");

#[derive(Debug)]
pub enum BoEvent {
    OpenPosition(PositionOpened),
    SettlePosition(PositionSettled),
}

pub fn parse_logs(logs: Vec<String>) -> Vec<BoEvent> {
    const OPEN_POSITION: &str = "Program log: Instruction: OpenPosition";
    const SETTLE_POSITION: &str = "Program log: Instruction: SettlePosition";
    const SETTLE_MULTIPLE_POSITONS: &str = "Program log: Instruction: SettleMultiplePositions";

    let Some(instruction) = logs
        .iter()
        .find(|log| {
            [OPEN_POSITION, SETTLE_POSITION, SETTLE_MULTIPLE_POSITONS].contains(&log.as_str())
        })
        .map(|ix| ix.as_str())
    else {
        return vec![];
    };

    logs.iter()
        .filter_map(|log| {
            if log.starts_with("Program data: ") {
                let log_data = log.strip_prefix("Program data: ")?;
                let bytes = &BASE64_STANDARD.decode(log_data).ok()?[DISCRIMINATOR_SIZE..];

                match instruction {
                    OPEN_POSITION => deserialize(bytes).map(BoEvent::OpenPosition),
                    SETTLE_POSITION | SETTLE_MULTIPLE_POSITONS => {
                        deserialize(bytes).map(BoEvent::SettlePosition)
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect()
}

fn deserialize<E: BorshDeserialize>(buffer: &[u8]) -> Option<E> {
    E::try_from_slice(buffer).ok()
}

const MARKET_ACCOUNT_SEED: &[u8] = b"binary_option_market_account";
const MASTER_ACCOUNT_SEED: &[u8] = b"binary_option_master_account";
const POSITION_ACCOUNT_SEED: &[u8] = b"binary_option_position_account";
const VAULT_ACCOUNT_SEED: &[u8] = b"binary_option_vault_account";
const POOL_ACCOUNT_SEED: &[u8] = b"binary_option_pool_account";

pub fn derive_market_pda(feed_id: [u8; 32]) -> Pubkey {
    Pubkey::find_program_address(&[MARKET_ACCOUNT_SEED, &feed_id], &PROGRAM_ID).0
}

pub fn derive_master_pda() -> Pubkey {
    Pubkey::find_program_address(&[MASTER_ACCOUNT_SEED], &PROGRAM_ID).0
}

pub fn derive_position_pda(user: &Pubkey, feed_id: [u8; 32], position_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            POSITION_ACCOUNT_SEED,
            user.as_ref(),
            &feed_id,
            position_id.as_ref(),
        ],
        &PROGRAM_ID,
    )
    .0
}

pub fn derive_vault_pda(mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[VAULT_ACCOUNT_SEED, mint.as_ref()], &PROGRAM_ID).0
}

pub fn derive_pool_pda(mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[POOL_ACCOUNT_SEED, mint.as_ref()], &PROGRAM_ID).0
}

pub fn derive_vault_ta_pda(vault: &Pubkey, mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[VAULT_ACCOUNT_SEED, vault.as_ref(), mint.as_ref()],
        &PROGRAM_ID,
    )
    .0
}

pub fn derive_pool_ta_pda(pool: &Pubkey, mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[POOL_ACCOUNT_SEED, pool.as_ref(), mint.as_ref()],
        &PROGRAM_ID,
    )
    .0
}
