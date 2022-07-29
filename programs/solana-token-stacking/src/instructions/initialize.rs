use crate::{
    errors::CustomErrors, events::PlatformInitializeEvent, state::Platform, BCDEV_DECIMALS,
    FCTR_DECIMALS,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = platform_authority,
        seeds = [b"platform"],
        bump,
        space = 8 + Platform::SPACE,
    )]
    platform: Account<'info, Platform>,
    #[account(mut)]
    platform_authority: Signer<'info>,
    /// CHECK:
    #[account(
        init,
        payer = platform_authority,
        seeds = [b"sol_vault"],
        bump,
        space = 0,
        owner = system_program.key(),
    )]
    sol_vault: AccountInfo<'info>,
    #[account(
        init,
        payer = platform_authority,
        seeds = [b"fctr_token_vault"],
        bump,
        space = 0,
        owner = system_program.key(),
    )]
    fctr_token_vault: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = platform_authority,
        seeds = [b"fctr_mint"],
        bump,
        mint::authority = platform,
        mint::decimals = FCTR_DECIMALS,
    )]
    fctr_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = platform_authority,
        seeds = [b"bcdev_mint"],
        bump,
        mint::authority = platform,
        mint::decimals = BCDEV_DECIMALS,
    )]
    bcdev_mint: Account<'info, Mint>,
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

pub fn initialize(
    ctx: Context<Initialize>,
    round_duration: u64,
    registration_price: u64,
) -> Result<()> {
    ctx.accounts.platform.bump = *ctx.bumps.get("platform").ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.platform.bump_sol_vault =
        *ctx.bumps.get("sol_vault").ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.platform.bump_fctr_token_vault = *ctx
        .bumps
        .get("fctr_token_vault")
        .ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.platform.bump_fctr_mint =
        *ctx.bumps.get("fctr_mint").ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.platform.bump_bcdev_mint =
        *ctx.bumps.get("bcdev_mint").ok_or(CustomErrors::EmptyBump)?;

    ctx.accounts.platform.round_duration = round_duration;
    ctx.accounts.platform.registration_price = registration_price;

    emit!(PlatformInitializeEvent {});

    Ok(())
}
