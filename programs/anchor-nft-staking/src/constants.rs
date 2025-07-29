use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

// Time constants
pub const SECONDS_PER_DAY: i64 = 86400;

// Account seeds
pub const CONFIG_SEED: &[u8] = b"config";
pub const USER_SEED: &[u8] = b"user";
pub const STAKE_SEED: &[u8] = b"stake";
pub const REWARDS_MINT_SEED: &[u8] = b"rewards_mint";
