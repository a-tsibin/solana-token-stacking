use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub bump: u8,
    pub bump_fctr_vault: u8,
    pub bump_bcdev_vault: u8,
    pub bump_receipt: u8,
    pub trust_program: bool
    pub authority: Pubkey,
}

impl User {
    pub const SPACE: usize = 1 + 1 + 1 + 1 + 1 + 32;
}
