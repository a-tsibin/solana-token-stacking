use crate::state::{GrantorHistoryRecord, GrantorRecord};
use anchor_lang::prelude::*;

const MAX_ACTIVE_GRANTORS: usize = 4;
const MAX_GRANTORS_HISTORY: usize = 100;

#[account]
#[derive(Default)]
pub struct Receipt {
    pub is_valid: bool,
    pub stake_duration: u64,
    pub stake_ts: u64,
    pub amount_deposited: u64,
    pub apr: f64,
    pub grantors: Vec<GrantorRecord>,
    pub grantors_history: Vec<GrantorHistoryRecord>,
    pub next_round_grantors: Vec<GrantorRecord>,
    pub authority: Pubkey,
}

impl Receipt {
    pub const SPACE: usize = 1
        + 8
        + 8
        + 8
        + 8
        + 32
        + (4 + MAX_ACTIVE_GRANTORS * GrantorRecord::SPACE)
        + (4 + MAX_ACTIVE_GRANTORS * GrantorRecord::SPACE)
        + (4 + MAX_GRANTORS_HISTORY * GrantorHistoryRecord::SPACE);
}
