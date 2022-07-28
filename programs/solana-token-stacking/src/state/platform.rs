use anchor_lang::prelude::*;

#[account]
pub struct Platform {
    pub bump: u8,
    pub bump_fctr_mint: u8,
    pub bump_bcdev_mint: u8,
    pub bump_sol_vault: u8,
    pub bump_fctr_token_vault: u8,
    pub round_start: u64,
    pub is_final: bool,
    pub round_duration: u64,
    pub fctr_token_total_amount: u64,
    pub bcdev_token_total_amount: u64,
    pub authority: Pubkey,
}

impl Platform {
    pub const SPACE: usize = 1 + 1 + 1 + 1 + 1 + 8 + 1 + 8 + 8 + 8 + 32;
}
