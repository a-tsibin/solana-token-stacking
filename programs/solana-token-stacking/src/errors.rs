use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("Can't get bump")]
    EmptyBump,
    #[msg("Platform active companies limit reached")]
    CompanyLimit,
    #[msg("The company you are trying to donate is closed")]
    DonationToClosedCompany,
    #[msg("Insufficient token amount")]
    InsufficientTokenAmount,
    #[msg("Illegal token vault owner")]
    IllegalTokenVaultOwner,
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
}
