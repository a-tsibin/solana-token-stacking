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
}

pub fn stake(ctx: Context<Stake>) -> Result<()> {
    let now: u64 = Clock::get()?.unix_timestamp as _;
    if ctx.accounts.receipt.authority != ctx.accounts.authority.key() {
        return err!(CustomErrors::InvalidReceiptAuthority);
    }
    if ctx.accounts.receipt.is_valid {
        return err!(CustomErrors::InvalidReceipt);
    }
    if ctx.accounts.platform.round_start + ctx.accounts.platform.round_duration < now {
        return err!(CustomErrors::NoActiveRound);
    }
    ctx.accounts.receipt.grantors = ctx.accounts.receipt.next_round_grantors.clone(); //std::mem::swap?
    ctx.accounts.receipt.next_round_grantors = Vec::new();

    let signer: &[&[&[u8]]] = &[&[b"platform", &[ctx.accounts.platform.bump]]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.fctr_vault.to_account_info(),
            to: ctx.accounts.platform_fctr_token_vault.to_account_info(),
            authority: ctx.accounts.platform.to_account_info(),
        },
        signer,
    );
    token::transfer(cpi_ctx, ctx.accounts.user.user_fctr_amount)?;

    ctx.accounts.receipt.is_valid = true;
    ctx.accounts.receipt.stake_ts = now;
    ctx.accounts.receipt.stake_duration =
        ctx.accounts.platform.round_start + ctx.accounts.platform.round_duration - now;
    ctx.accounts.receipt.amount_deposited = ctx.accounts.user.user_fctr_amount
        + ctx
            .accounts
            .receipt
            .grantors
            .iter()
            .fold(0, |sum, g| sum + g.amount);

    emit!(StakeEvent {
        amount: ctx.accounts.fctr_vault.amount
    });

    Ok(())
}
