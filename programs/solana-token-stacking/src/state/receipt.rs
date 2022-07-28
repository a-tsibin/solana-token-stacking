use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Receipt {
    pub is_valid: bool,
    pub stake_duration: u64,
    pub stake_ts: u64,
    pub amount_deposited: u64,
    pub authority: Pubkey,
}

impl Receipt {
    pub const SPACE: usize = 1 + 8 + 8 + 8 + 32;
}
