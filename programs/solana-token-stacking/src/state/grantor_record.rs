use anchor_lang::prelude::*;

#[account]
pub struct GrantorRecord {
    pub amount: u64,
    pub grant_duration: u64,
    pub grantor: Pubkey,
}

impl GrantorRecord {
    pub const SPACE: usize = 8 + 8 + 32;
}
