use crate::state::{GrantorHistoryRecord, GrantorRecord, Receipt};
use crate::{
    errors::CustomErrors,
    events::GrantEvent,
    state::{Platform, User},
};
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction()]
pub struct GrantTokens<'info> {
    #[account(mut, seeds = [b"receipt", authority.key().as_ref()], bump = user.bump_receipt)]
    receipt: Account<'info, Receipt>,
    #[account(seeds = [b"user", authority.key().as_ref()], bump = user.bump)]
    user: Account<'info, User>,
    #[account(mut, seeds = [b"fctr_vault", authority.key().as_ref()], bump = user.bump_fctr_vault)]
    fctr_vault: Account<'info, TokenAccount>,
    #[account(mut, address = user.authority)]
    authority: Signer<'info>,
    #[account(seeds = [b"user", confidant_authority.key().as_ref()], bump = confidant_user.bump)]
    confidant_user: Account<'info, User>,
    #[account(mut, seeds = [b"receipt", confidant_authority.key().as_ref()], bump = confidant_user.bump_receipt)]
    confidant_receipt: Account<'info, Receipt>,
    /// CHECK:
    confidant_authority: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"platform"], bump = platform.bump)]
    platform: Account<'info, Platform>,
    #[account(mut, seeds = [b"fctr_token_vault"], bump = platform.bump_fctr_token_vault)]
    platform_fctr_token_vault: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    clock: Sysvar<'info, Clock>,
}

pub fn grant_tokens(ctx: Context<GrantTokens>, amount: u64) -> Result<()> {
    let now: u64 = ctx.accounts.clock.unix_timestamp as _;
    if (ctx.accounts.receipt.authority != ctx.accounts.authority.key())
        || (ctx.accounts.confidant_receipt.authority != ctx.accounts.confidant_authority.key())
    {
        return err!(CustomErrors::InvalidReceiptAuthority);
    }
    if !ctx.accounts.confidant_user.grant_program || !ctx.accounts.user.grant_program {
        return err!(CustomErrors::GrantProgramError);
    }
    //todo: We can create another list of grantors in receipt to deal with cases,
    // when new round started, but confidant doesn't unstake previous round
    // and user want to grant tokens to him.
    if ctx.accounts.receipt.is_valid && ctx.accounts.receipt.stake_ts < now {
        return err!(CustomErrors::InvalidReceipt);
    }
    let amount_ratio = ctx.accounts.user.total_fctr_amount as f64
        / ctx.accounts.confidant_user.total_fctr_amount as f64;
    if ctx
        .accounts
        .confidant_receipt
        .grantors
        .iter()
        .any(|g| g.grantor == ctx.accounts.fctr_vault.key())
        || ctx.accounts.confidant_receipt.grantors.len() >= 4
        || (0.5 <= amount_ratio && amount_ratio <= 2.0)
    {
        return err!(CustomErrors::TokenGrantError);
    }
    if ctx
        .accounts
        .confidant_receipt
        .grantors_history
        .iter()
        .any(|g| g.grantor == ctx.accounts.fctr_vault.key())
    {
        return err!(CustomErrors::GrantCooldown);
    }

    ctx.accounts.receipt.apr =
        ctx.accounts.receipt.apr * ctx.accounts.fctr_vault.amount as f64 / amount as f64;

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.fctr_vault.to_account_info(),
            to: ctx.accounts.platform_fctr_token_vault.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
    );
    token::transfer(cpi_ctx, amount)?;

    ctx.accounts.confidant_receipt.amount_deposited += amount;
    ctx.accounts.confidant_receipt.grantors.push(GrantorRecord {
        amount,
        grant_duration: ctx.accounts.platform.round_start + ctx.accounts.platform.round_duration
            - now,
        grantor: ctx.accounts.user.key(),
    });
    ctx.accounts
        .confidant_receipt
        .grantors_history
        .push(GrantorHistoryRecord {
            grantor: ctx.accounts.user.key(),
            grant_ts: now,
        });

    emit!(GrantEvent {
        from: ctx.accounts.user.key(),
        to: ctx.accounts.confidant_user.key(),
        amount,
    });

    Ok(())
}
