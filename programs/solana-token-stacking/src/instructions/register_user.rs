use crate::errors::CustomErrors;
use crate::state::{Platform, Receipt};
use crate::{events::UserRegisteredEvent, state::User};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [b"user", authority.key().as_ref()],
        bump,
        space = 8 + User::SPACE,
    )]
    user: Account<'info, User>,
    #[account(
        init,
        payer = authority,
        seeds = [b"fctr_vault", authority.key().as_ref()],
        bump,
        space = 0,
        owner = authority.key(),
    )]
    fctr_vault: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = authority,
        seeds = [b"bcdev_vault", authority.key().as_ref()],
        bump,
        space = 0,
        owner = authority.key(),
    )]
    bcdev_vault: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = authority,
        seeds = [b"receipt", authority.key().as_ref()],
        bump,
        space = 8 + Receipt::SPACE,
    )]
    receipt: Account<'info, Receipt>,
    #[account(mut)]
    authority: Signer<'info>,
    /// CHECK:
    #[account(mut, seeds = [b"sol_vault"], bump = platform.bump_sol_vault)]
    sol_vault: AccountInfo<'info>,
    #[account(mut, seeds = [b"platform"], bump = platform.bump)]
    platform: Account<'info, Platform>,
    system_program: Program<'info, System>,
}

pub fn register_user(ctx: Context<RegisterUser>, participate_in_grant_program: bool) -> Result<()> {
    ctx.accounts.user.bump = *ctx.bumps.get("user").ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.user.bump_fctr_vault =
        *ctx.bumps.get("fctr_vault").ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.user.bump_bcdev_vault = *ctx
        .bumps
        .get("bcdev_vault")
        .ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.user.bump_receipt = *ctx.bumps.get("receipt").ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.user.authority = ctx.accounts.authority.key();
    ctx.accounts.user.grant_program = participate_in_grant_program;
    ctx.accounts.receipt.authority = ctx.accounts.authority.key();
    ctx.accounts.receipt.apr = 0.01;

    invoke(
        &system_instruction::transfer(
            ctx.accounts.authority.key,
            ctx.accounts.sol_vault.key,
            ctx.accounts.platform.registration_price,
        ),
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.sol_vault.to_account_info(),
        ],
    )?;

    emit!(UserRegisteredEvent {
        user: ctx.accounts.authority.key()
    });

    Ok(())
}
