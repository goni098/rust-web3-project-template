use solana_sdk::pubkey::Pubkey;

use crate::bo::program::PROGRAM_ID;

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
