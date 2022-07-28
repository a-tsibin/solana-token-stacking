use crate::state::Receipt;
use crate::{
    errors::CustomErrors,
    events::UnstakeEvent,
    state::{Platform, User},
};
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut, seeds = [b"receipt", authority.key().as_ref()], bump = user.bump_receipt)]
    receipt: Account<'info, Receipt>,
    #[account(seeds = [b"user", authority.key().as_ref()], bump = user.bump)]
    user: Account<'info, User>,
    #[account(mut, seeds = [b"fctr_vault", authority.key().as_ref()], bump = user.bump_fctr_vault)]
    fctr_vault: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"bcdev_vault", authority.key().as_ref()], bump = user.bump_bcdev_vault)]
    bcdev_vault: Account<'info, TokenAccount>,
    #[account(mut, address = user.authority)]
    authority: Signer<'info>,
    #[account(mut, seeds = [b"platform"], bump = platform.bump)]
    platform: Account<'info, Platform>,
    #[account(mut, seeds = [b"fctr_token_vault"], bump = platform.bump_fctr_token_vault)]
    platform_fctr_token_vault: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"bcdev_mint"], bump = platform.bump_bcdev_mint)]
    bcdev_mint: Account<'info, Mint>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
    if ctx.accounts.receipt.authority != ctx.accounts.authority.key() {
        return err!(CustomErrors::InvalidReceiptAuthority);
    }
    if !ctx.accounts.receipt.is_valid {
        return err!(CustomErrors::InvalidReceipt);
    }
    if ctx.accounts.platform.round_start < ctx.accounts.receipt.stake_ts {
        return err!(CustomErrors::RoundStillGoing);
    }

    let signer: &[&[&[u8]]] = &[&[b"platform", &[ctx.accounts.platform.bump]]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.platform_fctr_token_vault.to_account_info(),
            to: ctx.accounts.fctr_vault.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
        signer,
    );
    token::transfer(cpi_ctx, ctx.accounts.receipt.amount_deposited)?;

    let reward_percentage =
        ctx.accounts.receipt.stake_duration as f64 / ctx.accounts.platform.round_duration as f64;
    let reward_amount: u64 =
        (ctx.accounts.receipt.amount_deposited as f64 * reward_percentage).round() as _;

    let mint_cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.bcdev_mint.to_account_info(),
            to: ctx.accounts.bcdev_vault.to_account_info(),
            authority: ctx.accounts.platform.to_account_info(),
        },
        signer,
    );
    token::mint_to(mint_cpi_ctx, reward_amount)?;

    ctx.accounts.receipt.is_valid = false;

    emit!(UnstakeEvent {});

    Ok(())
}
