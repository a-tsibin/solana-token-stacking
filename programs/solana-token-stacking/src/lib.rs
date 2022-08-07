use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const FCTR_DECIMALS: u8 = 12;
const BCDEV_DECIMALS: u8 = 18;

#[program]
pub mod solana_token_stacking {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        round_duration: u64,
        registration_price: u64,
    ) -> Result<()> {
        initialize::initialize(ctx, round_duration, registration_price)
    }

    pub fn register_user(
        ctx: Context<RegisterUser>,
        participate_in_grant_program: bool,
    ) -> Result<()> {
        register_user::register_user(ctx, participate_in_grant_program)
    }

    pub fn start_round(ctx: Context<StartRound>, is_final: bool) -> Result<()> {
        start_round::start_round(ctx, is_final)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw::withdraw(ctx)
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, token_amount: u64) -> Result<()> {
        buy_tokens::buy_tokens(ctx, token_amount)
    }

    pub fn sell_fctr_tokens(ctx: Context<SellFctrTokens>) -> Result<()> {
        sell_fctr_tokens::sell_fctr_tokens(ctx)
    }

    pub fn sell_bcdev_tokens(ctx: Context<SellBcdevTokens>, amount: u64) -> Result<()> {
        sell_bcdev_tokens::sell_bcdev_tokens(ctx, amount)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount: u64) -> Result<()> {
        add_liquidity::add_liquidity(ctx, amount)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        stake::stake(ctx)
    }

    pub fn unstake<'info>(ctx: Context<'_, '_, '_, 'info, Unstake<'info>>) -> Result<()> {
        unstake::unstake(ctx)
    }

    pub fn grant_tokens(ctx: Context<GrantTokens>, amount: u64) -> Result<()> {
        grant_tokens::grant_tokens(ctx, amount)
    }
}
