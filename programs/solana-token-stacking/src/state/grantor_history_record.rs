use anchor_lang::prelude::*;

#[account]
pub struct GrantorHistoryRecord {
    pub grant_ts: u64,
    pub grantor: Pubkey,
}

impl GrantorHistoryRecord {
    pub const SPACE: usize = 8 + 32;
}
