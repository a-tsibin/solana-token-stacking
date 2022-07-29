use anchor_lang::prelude::*;

#[event]
pub struct PlatformInitializeEvent {}

#[event]
pub struct UserRegisteredEvent {
    pub user: Pubkey,
}

#[event]
pub struct WithdrawEvent {}

#[event]
pub struct BuyFctrTokensEvent {
    pub amount: u64,
}

#[event]
pub struct SellFctrTokensEvent {
    pub amount: u64,
}

#[event]
pub struct SellBcdevTokensEvent {
    pub amount: u64,
}

#[event]
pub struct RoundStartEvent {
    pub is_final: bool,
}

#[event]
pub struct LiquidityAddedEvent {
    pub amount: u64,
}

#[event]
pub struct StakeEvent {
    pub amount: u64,
}

#[event]
pub struct UnstakeEvent {}

#[event]
pub struct GrantEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

#[event]
pub struct ClaimEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}
