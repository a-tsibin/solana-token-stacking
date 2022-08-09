use crate::state::Receipt;
use crate::{
    errors::CustomErrors,
    events::ClaimEvent,
    state::{Platform, User},
};
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction()]
pub struct ClaimTokens<'info> {
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

pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
    if (ctx.accounts.receipt.authority != ctx.accounts.authority.key())
        || (ctx.accounts.confidant_receipt.authority != ctx.accounts.confidant_authority.key())
    {
        return err!(CustomErrors::InvalidReceiptAuthority);
    }
    let granted_amount = ctx
        .accounts
        .confidant_receipt
        .grantors
        .iter()
        .find(|g| g.grantor == ctx.accounts.user.key())
        .ok_or(CustomErrors::GrantorNotFound)?
        .amount;

    ctx.accounts
        .confidant_receipt
        .grantors
        .retain(|g| g.grantor != ctx.accounts.user.key());
    ctx.accounts.confidant_receipt.amount_deposited -= granted_amount;

    let signer: &[&[&[u8]]] = &[&[b"platform", &[ctx.accounts.platform.bump]]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.platform_fctr_token_vault.to_account_info(),
            to: ctx.accounts.fctr_vault.to_account_info(),
            authority: ctx.accounts.platform.to_account_info(),
        },
        signer,
    );
    token::transfer(cpi_ctx, granted_amount)?;

    emit!(ClaimEvent {
        from: ctx.accounts.user.key(),
        to: ctx.accounts.confidant_user.key(),
        amount: granted_amount,
    });

    Ok(())
}
