#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
pub mod bo {
    pub mod account {
        mod market_account {
            use borsh::BorshDeserialize;
            use solana_sdk::pubkey::Pubkey;
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
            #[automatically_derived]
            impl borsh::de::BorshDeserialize for MarketAccount {
                fn deserialize_reader<__R: borsh::io::Read>(
                    reader: &mut __R,
                ) -> ::core::result::Result<Self, borsh::io::Error> {
                    Ok(Self {
                        operator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        back_authority: borsh::BorshDeserialize::deserialize_reader(
                            reader,
                        )?,
                        bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        min_settle_delay_epochs_secs: borsh::BorshDeserialize::deserialize_reader(
                            reader,
                        )?,
                        max_settle_delay_epochs_secs: borsh::BorshDeserialize::deserialize_reader(
                            reader,
                        )?,
                        fee_bps: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        min_stake: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        max_stake: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        min_premium_bps: borsh::BorshDeserialize::deserialize_reader(
                            reader,
                        )?,
                        call_lambda: borsh::BorshDeserialize::deserialize_reader(
                            reader,
                        )?,
                        put_lambda: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        vega_buffer: borsh::BorshDeserialize::deserialize_reader(
                            reader,
                        )?,
                        feed_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        staleness_max_sec: borsh::BorshDeserialize::deserialize_reader(
                            reader,
                        )?,
                        price_exponent: borsh::BorshDeserialize::deserialize_reader(
                            reader,
                        )?,
                        last_price: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        last_ts: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        sigma2: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        min_sigma2: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        max_sigma2: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        half_life_secs: borsh::BorshDeserialize::deserialize_reader(
                            reader,
                        )?,
                        vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        pool: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        treasury: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        paused: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    })
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for MarketAccount {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "operator",
                        "back_authority",
                        "bump",
                        "min_settle_delay_epochs_secs",
                        "max_settle_delay_epochs_secs",
                        "fee_bps",
                        "min_stake",
                        "max_stake",
                        "min_premium_bps",
                        "call_lambda",
                        "put_lambda",
                        "vega_buffer",
                        "feed_id",
                        "staleness_max_sec",
                        "price_exponent",
                        "last_price",
                        "last_ts",
                        "sigma2",
                        "min_sigma2",
                        "max_sigma2",
                        "half_life_secs",
                        "vault",
                        "pool",
                        "treasury",
                        "paused",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.operator,
                        &self.back_authority,
                        &self.bump,
                        &self.min_settle_delay_epochs_secs,
                        &self.max_settle_delay_epochs_secs,
                        &self.fee_bps,
                        &self.min_stake,
                        &self.max_stake,
                        &self.min_premium_bps,
                        &self.call_lambda,
                        &self.put_lambda,
                        &self.vega_buffer,
                        &self.feed_id,
                        &self.staleness_max_sec,
                        &self.price_exponent,
                        &self.last_price,
                        &self.last_ts,
                        &self.sigma2,
                        &self.min_sigma2,
                        &self.max_sigma2,
                        &self.half_life_secs,
                        &self.vault,
                        &self.pool,
                        &self.treasury,
                        &&self.paused,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "MarketAccount",
                        names,
                        values,
                    )
                }
            }
        }
        pub use market_account::*;
    }
    pub mod event {
        mod postion_opened {
            use borsh::BorshDeserialize;
            use solana_sdk::pubkey::Pubkey;
            use crate::bo::event::{PositionMode, Side};
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
            #[automatically_derived]
            impl ::core::fmt::Debug for PositionOpened {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "id",
                        "feed_id",
                        "user",
                        "side",
                        "amount",
                        "premium",
                        "premium_amount",
                        "boundary_price",
                        "spot_price",
                        "multiplier",
                        "start_time",
                        "end_time",
                        "mode",
                        "old_sigma2",
                        "new_sigma2",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.id,
                        &self.feed_id,
                        &self.user,
                        &self.side,
                        &self.amount,
                        &self.premium,
                        &self.premium_amount,
                        &self.boundary_price,
                        &self.spot_price,
                        &self.multiplier,
                        &self.start_time,
                        &self.end_time,
                        &self.mode,
                        &self.old_sigma2,
                        &&self.new_sigma2,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "PositionOpened",
                        names,
                        values,
                    )
                }
            }
            #[automatically_derived]
            impl borsh::de::BorshDeserialize for PositionOpened {
                fn deserialize_reader<__R: borsh::io::Read>(
                    reader: &mut __R,
                ) -> ::core::result::Result<Self, borsh::io::Error> {
                    Ok(Self {
                        id: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        feed_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        user: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        side: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        premium: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        premium_amount: borsh::BorshDeserialize::deserialize_reader(
                            reader,
                        )?,
                        boundary_price: borsh::BorshDeserialize::deserialize_reader(
                            reader,
                        )?,
                        spot_price: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        multiplier: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        start_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        end_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        mode: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        old_sigma2: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        new_sigma2: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    })
                }
            }
        }
        mod postion_settled {
            use borsh::BorshDeserialize;
            use solana_sdk::pubkey::Pubkey;
            use crate::bo::event::{PositionResult, Side};
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
            #[automatically_derived]
            impl ::core::fmt::Debug for PositionSettled {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "id",
                        "user",
                        "side",
                        "boundary_price",
                        "spot_price",
                        "multiplier",
                        "final_price",
                        "result",
                        "payout",
                        "premium",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.id,
                        &self.user,
                        &self.side,
                        &self.boundary_price,
                        &self.spot_price,
                        &self.multiplier,
                        &self.final_price,
                        &self.result,
                        &self.payout,
                        &&self.premium,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "PositionSettled",
                        names,
                        values,
                    )
                }
            }
            #[automatically_derived]
            impl borsh::de::BorshDeserialize for PositionSettled {
                fn deserialize_reader<__R: borsh::io::Read>(
                    reader: &mut __R,
                ) -> ::core::result::Result<Self, borsh::io::Error> {
                    Ok(Self {
                        id: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        user: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        side: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        boundary_price: borsh::BorshDeserialize::deserialize_reader(
                            reader,
                        )?,
                        spot_price: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        multiplier: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        final_price: borsh::BorshDeserialize::deserialize_reader(
                            reader,
                        )?,
                        result: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        payout: borsh::BorshDeserialize::deserialize_reader(reader)?,
                        premium: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    })
                }
            }
        }
        use borsh::{BorshDeserialize, BorshSerialize};
        pub use postion_opened::*;
        pub use postion_settled::*;
        pub enum PositionResult {
            Won,
            Lose,
            Draw,
            Refund,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PositionResult {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        PositionResult::Won => "Won",
                        PositionResult::Lose => "Lose",
                        PositionResult::Draw => "Draw",
                        PositionResult::Refund => "Refund",
                    },
                )
            }
        }
        #[automatically_derived]
        impl borsh::de::BorshDeserialize for PositionResult {
            fn deserialize_reader<__R: borsh::io::Read>(
                reader: &mut __R,
            ) -> ::core::result::Result<Self, borsh::io::Error> {
                let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(
                    reader,
                )?;
                <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
            }
        }
        #[automatically_derived]
        impl borsh::de::EnumExt for PositionResult {
            fn deserialize_variant<__R: borsh::io::Read>(
                reader: &mut __R,
                variant_tag: u8,
            ) -> ::core::result::Result<Self, borsh::io::Error> {
                let mut return_value = if variant_tag == 0u8 {
                    PositionResult::Won
                } else if variant_tag == 1u8 {
                    PositionResult::Lose
                } else if variant_tag == 2u8 {
                    PositionResult::Draw
                } else if variant_tag == 3u8 {
                    PositionResult::Refund
                } else {
                    return Err(
                        borsh::io::Error::new(
                            borsh::io::ErrorKind::InvalidData,
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("Unexpected variant tag: {0:?}", variant_tag),
                                )
                            }),
                        ),
                    )
                };
                Ok(return_value)
            }
        }
        pub enum Side {
            Call,
            Put,
        }
        #[automatically_derived]
        impl borsh::de::BorshDeserialize for Side {
            fn deserialize_reader<__R: borsh::io::Read>(
                reader: &mut __R,
            ) -> ::core::result::Result<Self, borsh::io::Error> {
                let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(
                    reader,
                )?;
                <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
            }
        }
        #[automatically_derived]
        impl borsh::de::EnumExt for Side {
            fn deserialize_variant<__R: borsh::io::Read>(
                reader: &mut __R,
                variant_tag: u8,
            ) -> ::core::result::Result<Self, borsh::io::Error> {
                let mut return_value = if variant_tag == 0u8 {
                    Side::Call
                } else if variant_tag == 1u8 {
                    Side::Put
                } else {
                    return Err(
                        borsh::io::Error::new(
                            borsh::io::ErrorKind::InvalidData,
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("Unexpected variant tag: {0:?}", variant_tag),
                                )
                            }),
                        ),
                    )
                };
                Ok(return_value)
            }
        }
        #[automatically_derived]
        impl borsh::ser::BorshSerialize for Side {
            fn serialize<__W: borsh::io::Write>(
                &self,
                writer: &mut __W,
            ) -> ::core::result::Result<(), borsh::io::Error> {
                let variant_idx: u8 = match self {
                    Side::Call => 0u8,
                    Side::Put => 1u8,
                };
                writer.write_all(&variant_idx.to_le_bytes())?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Side {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Side::Call => "Call",
                        Side::Put => "Put",
                    },
                )
            }
        }
        pub enum PositionMode {
            Single,
            Batch,
        }
        #[automatically_derived]
        impl borsh::de::BorshDeserialize for PositionMode {
            fn deserialize_reader<__R: borsh::io::Read>(
                reader: &mut __R,
            ) -> ::core::result::Result<Self, borsh::io::Error> {
                let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(
                    reader,
                )?;
                <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
            }
        }
        #[automatically_derived]
        impl borsh::de::EnumExt for PositionMode {
            fn deserialize_variant<__R: borsh::io::Read>(
                reader: &mut __R,
                variant_tag: u8,
            ) -> ::core::result::Result<Self, borsh::io::Error> {
                let mut return_value = if variant_tag == 0u8 {
                    PositionMode::Single
                } else if variant_tag == 1u8 {
                    PositionMode::Batch
                } else {
                    return Err(
                        borsh::io::Error::new(
                            borsh::io::ErrorKind::InvalidData,
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("Unexpected variant tag: {0:?}", variant_tag),
                                )
                            }),
                        ),
                    )
                };
                Ok(return_value)
            }
        }
        #[automatically_derived]
        impl borsh::ser::BorshSerialize for PositionMode {
            fn serialize<__W: borsh::io::Write>(
                &self,
                writer: &mut __W,
            ) -> ::core::result::Result<(), borsh::io::Error> {
                let variant_idx: u8 = match self {
                    PositionMode::Single => 0u8,
                    PositionMode::Batch => 1u8,
                };
                writer.write_all(&variant_idx.to_le_bytes())?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PositionMode {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        PositionMode::Single => "Single",
                        PositionMode::Batch => "Batch",
                    },
                )
            }
        }
    }
    pub mod instruction {
        mod open_position {
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
                    derive_market_pda, derive_pool_pda, derive_position_pda,
                    derive_vault_pda, derive_vault_ta_pda,
                },
                program::PROGRAM_ID,
            };
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
            #[automatically_derived]
            impl borsh::ser::BorshSerialize for OpenPositionArgs {
                fn serialize<__W: borsh::io::Write>(
                    &self,
                    writer: &mut __W,
                ) -> ::core::result::Result<(), borsh::io::Error> {
                    borsh::BorshSerialize::serialize(&self._id, writer)?;
                    borsh::BorshSerialize::serialize(&self.side, writer)?;
                    borsh::BorshSerialize::serialize(&self.amount, writer)?;
                    borsh::BorshSerialize::serialize(&self.position_id, writer)?;
                    borsh::BorshSerialize::serialize(
                        &self.settle_delay_epochs_secs,
                        writer,
                    )?;
                    borsh::BorshSerialize::serialize(&self.boundary_price, writer)?;
                    borsh::BorshSerialize::serialize(&self.price, writer)?;
                    borsh::BorshSerialize::serialize(&self.price_publish_time, writer)?;
                    borsh::BorshSerialize::serialize(&self.mode, writer)?;
                    Ok(())
                }
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
                let user_ata = get_associated_token_address_with_program_id(
                    user,
                    mint,
                    token_program_id,
                );
                let vault = derive_vault_pda(mint);
                let pool = derive_pool_pda(mint);
                let vault_ta = derive_vault_ta_pda(&vault, mint);
                let accounts = <[_]>::into_vec(
                    ::alloc::boxed::box_new([
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
                    ]),
                );
                let discriminator: [u8; 8] = [135, 128, 47, 77, 15, 152, 240, 49];
                let mut data = discriminator.to_vec();
                args.serialize(&mut data)?;
                Ok(Instruction {
                    accounts,
                    data,
                    program_id: PROGRAM_ID,
                })
            }
        }
        pub use open_position::*;
    }
    pub mod pda {
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
        pub fn derive_position_pda(
            user: &Pubkey,
            feed_id: [u8; 32],
            position_id: &Pubkey,
        ) -> Pubkey {
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
            Pubkey::find_program_address(
                    &[VAULT_ACCOUNT_SEED, mint.as_ref()],
                    &PROGRAM_ID,
                )
                .0
        }
        pub fn derive_pool_pda(mint: &Pubkey) -> Pubkey {
            Pubkey::find_program_address(
                    &[POOL_ACCOUNT_SEED, mint.as_ref()],
                    &PROGRAM_ID,
                )
                .0
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
    }
    pub mod program {
        use solana_sdk::pubkey;
        use solana_sdk::pubkey::Pubkey;
        use crate::bo::event::{PositionOpened, PositionSettled};
        pub const PROGRAM_ID: Pubkey = ::solana_address::Address::from_str_const(
            "HF1uuMHBmtaCYqhFx2wCFYhkAim8uLaxpMUeWmeTXtD9",
        );
        pub enum BoEvent {
            OpenPosition(PositionOpened),
            SettlePosition(PositionSettled),
        }
        static OPENPOSITION_DISC: [u8; 8] = [
            237u8, 175u8, 243u8, 230u8, 147u8, 117u8, 101u8, 121u8,
        ];
        static SETTLEPOSITION_DISC: [u8; 8] = [
            75u8, 100u8, 92u8, 189u8, 245u8, 116u8, 252u8, 221u8,
        ];
        impl BoEvent {
            pub fn from_logs(logs: &[String]) -> Vec<Self> {
                use borsh::BorshDeserialize;
                use base64::{Engine, prelude::BASE64_STANDARD};
                logs.into_iter()
                    .filter_map(|log| {
                        let data = log.strip_prefix("Program data: ")?;
                        let bytes = BASE64_STANDARD.decode(data).ok()?;
                        let (disc, body) = bytes.split_at(8u8 as usize);
                        match disc {
                            d if d == OPENPOSITION_DISC => {
                                Some(
                                    BoEvent::OpenPosition(
                                        PositionOpened::try_from_slice(body).ok()?,
                                    ),
                                )
                            }
                            d if d == SETTLEPOSITION_DISC => {
                                Some(
                                    BoEvent::SettlePosition(
                                        PositionSettled::try_from_slice(body).ok()?,
                                    ),
                                )
                            }
                            _ => None,
                        }
                    })
                    .collect()
            }
        }
    }
}
pub mod fetcher {
    use borsh::BorshDeserialize;
    use shared::result::Rs;
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::pubkey::Pubkey;
    use crate::DISCRIMINATOR_SIZE;
    pub async fn fetch_account<T: BorshDeserialize>(
        client: &RpcClient,
        pubkey: &Pubkey,
    ) -> Rs<T> {
        let data = client.get_account_data(pubkey).await?;
        let buf: &mut &[u8] = &mut &data.as_slice()[DISCRIMINATOR_SIZE..];
        let acc = T::deserialize(buf)?;
        Ok(acc)
    }
}
const DISCRIMINATOR_SIZE: usize = 8;
