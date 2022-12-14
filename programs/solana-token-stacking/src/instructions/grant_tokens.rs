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
    platform: Box<Account<'info, Platform>>,
    #[account(mut, seeds = [b"fctr_token_vault"], bump = platform.bump_fctr_token_vault)]
    platform_fctr_token_vault: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

pub fn grant_tokens(ctx: Context<GrantTokens>, amount: u64) -> Result<()> {
    let now: u64 = Clock::get()?.unix_timestamp as _;
    if (ctx.accounts.receipt.authority != ctx.accounts.authority.key())
        || (ctx.accounts.confidant_receipt.authority != ctx.accounts.confidant_user.authority)
    {
        return err!(CustomErrors::InvalidReceiptAuthority);
    }
    if !ctx.accounts.confidant_user.grant_program || !ctx.accounts.user.grant_program {
        return err!(CustomErrors::GrantProgramError);
    }
    let grant_duration = if ctx.accounts.confidant_receipt.stake_ts > now {
        ctx.accounts.platform.round_start + ctx.accounts.platform.round_duration - now
    } else {
        ctx.accounts.platform.round_duration
    };

    msg!(
        "Stake: {}. Start: {}. End: {}. Now: {}",
        ctx.accounts.confidant_receipt.stake_ts,
        ctx.accounts.platform.round_start,
        ctx.accounts.platform.round_start + ctx.accounts.platform.round_duration,
        now
    );
    let grantors_list = if ctx.accounts.confidant_receipt.round_ends > now {
        &mut ctx.accounts.confidant_receipt.grantors
    } else {
        &mut ctx.accounts.confidant_receipt.next_round_grantors
    };

    let amount_ratio = ctx.accounts.user.user_fctr_amount as f64
        / ctx.accounts.confidant_user.user_fctr_amount as f64;
    if grantors_list
        .iter()
        .any(|g| g.grantor == ctx.accounts.fctr_vault.key())
        || grantors_list.len() >= 4
        || amount_ratio < 0.5
        || 2.0 < amount_ratio
    {
        return err!(CustomErrors::TokenGrantError);
    }
    grantors_list.push(GrantorRecord {
        amount,
        grant_duration,
        grantor: ctx.accounts.user.key(),
    });
    if ctx
        .accounts
        .confidant_receipt
        .grantors_history
        .iter()
        .any(|g| g.grantor == ctx.accounts.fctr_vault.key())
    {
        return err!(CustomErrors::GrantCooldown);
    }

    ctx.accounts.receipt.apr = ctx.accounts.receipt.apr * ctx.accounts.fctr_vault.amount as f64
        / (ctx.accounts.fctr_vault.amount - amount) as f64;

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
    token::transfer(cpi_ctx, amount)?;

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
