use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("Can't get bump")]
    EmptyBump,
    #[msg("Withdraw conditions are unsatisfied")]
    WithdrawConditions,
    #[msg("Invalid amount to buy. The minimum amount to buy is 10")]
    InvalidBuyAmount,
    #[msg("Stacking campaign finished")]
    StackingFinished,
    #[msg("Round already started")]
    RoundAlreadyStarted,
    #[msg("Invalid receipt authority")]
    InvalidReceiptAuthority,
    #[msg("Invalid receipt")]
    InvalidReceipt,
    #[msg("No active round")]
    NoActiveRound,
    #[msg("Round still going")]
    RoundStillGoing,
    #[msg("You are already granted tokens to this user")]
    TokensAlreadyGranted,
    #[msg("Grant cooldown")]
    GrantCooldown,
    #[msg("Maximum grantors count reached")]
    GrantorsCountLimit,
    #[msg("Token grant error")]
    TokenGrantError,
    #[msg("Grantor not found")]
    GrantorNotFound,
    #[msg("User doesn't participate in grant progrma")]
    GrantProgramError,
    #[msg("Invalid grantors list")]
    InvalidGrantorsList,
}
