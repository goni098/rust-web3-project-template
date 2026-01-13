use borsh::BorshSerialize;
use shared::result::Rs;
use solana_sdk::{
    message::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_system_interface::program::ID as SYSTEM_PROGRAM_ID;
use spl_associated_token_account_interface::address::get_associated_token_address_with_program_id;

use crate::bo::{
    event::{PositionMode, Side},
    pda::{
        derive_market_pda, derive_pool_pda, derive_position_pda, derive_vault_pda,
        derive_vault_ta_pda,
    },
    program::PROGRAM_ID,
};

#[derive(BorshSerialize)]
pub struct OpenPositionArgs {
    pub _id: [u8; 32],
    pub side: Side,
    pub amount: u64,
    pub position_id: [u8; 32],
    pub settle_delay_epochs_secs: u32,
    pub boundary_price: i64,
    pub price: i64,
    pub price_publish_time: i64,
    pub mode: PositionMode,
}

pub fn open_position(
    user: &Pubkey,
    payer: &Pubkey,
    back_authority: &Pubkey,
    mint: &Pubkey,
    position_id: &Pubkey,
    token_program_id: &Pubkey,
    args: OpenPositionArgs,
) -> Rs<Instruction> {
    let feed_id = args._id;

    let market = derive_market_pda(feed_id);
    let position = derive_position_pda(user, feed_id, position_id);
    let user_ata = get_associated_token_address_with_program_id(user, mint, token_program_id);
    let vault = derive_vault_pda(mint);
    let pool = derive_pool_pda(mint);
    let vault_ta = derive_vault_ta_pda(&vault, mint);

    let accounts = vec![
        AccountMeta::new(market, false),
        AccountMeta::new(position, false),
        AccountMeta::new_readonly(*mint, false),
        AccountMeta::new(user_ata, false),
        AccountMeta::new(vault, false),
        AccountMeta::new(pool, false),
        AccountMeta::new(vault_ta, false),
        AccountMeta::new_readonly(*user, true),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*back_authority, true),
        AccountMeta::new_readonly(*token_program_id, false),
        AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
    ];

    let discriminator: [u8; 8] = [135, 128, 47, 77, 15, 152, 240, 49];

    let mut data = discriminator.to_vec();
    args.serialize(&mut data)?;

    Ok(Instruction {
        accounts,
        data,
        program_id: PROGRAM_ID,
    })
}
