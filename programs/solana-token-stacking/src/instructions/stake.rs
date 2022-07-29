use crate::state::Receipt;
use crate::{
    errors::CustomErrors,
    events::StakeEvent,
    state::{Platform, User},
};
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut, seeds = [b"receipt", authority.key().as_ref()], bump = user.bump_receipt)]
    receipt: Account<'info, Receipt>,
    #[account(seeds = [b"user", authority.key().as_ref()], bump = user.bump)]
    user: Account<'info, User>,
    #[account(mut, seeds = [b"fctr_vault", authority.key().as_ref()], bump = user.bump_fctr_vault)]
    fctr_vault: Account<'info, TokenAccount>,
    #[account(mut, address = user.authority)]
    authority: Signer<'info>,
    #[account(mut, seeds = [b"platform"], bump = platform.bump)]
    platform: Account<'info, Platform>,
    #[account(mut, seeds = [b"fctr_token_vault"], bump = platform.bump_fctr_token_vault)]
    platform_fctr_token_vault: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    clock: Sysvar<'info, Clock>,
}

pub fn stake(ctx: Context<Stake>) -> Result<()> {
    let now: u64 = ctx.accounts.clock.unix_timestamp as _;
    if ctx.accounts.receipt.authority != ctx.accounts.authority.key() {
        return err!(CustomErrors::InvalidReceiptAuthority);
    }
    if ctx.accounts.receipt.is_valid {
        return err!(CustomErrors::InvalidReceipt);
    }
    if ctx.accounts.platform.round_start + ctx.accounts.platform.round_duration < now {
        return err!(CustomErrors::NoActiveRound);
    }

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.fctr_vault.to_account_info(),
            to: ctx.accounts.platform_fctr_token_vault.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
    );
    token::transfer(cpi_ctx, ctx.accounts.user.total_fctr_amount)?;

    ctx.accounts.receipt.is_valid = true;
    ctx.accounts.receipt.stake_ts = now;
    ctx.accounts.receipt.stake_duration =
        ctx.accounts.platform.round_start + ctx.accounts.platform.round_duration - now;
    ctx.accounts.receipt.amount_deposited = ctx.accounts.user.total_fctr_amount;
    ctx.accounts.receipt.grantors = Vec::new();

    emit!(StakeEvent {
        amount: ctx.accounts.fctr_vault.amount
    });

    Ok(())
}
