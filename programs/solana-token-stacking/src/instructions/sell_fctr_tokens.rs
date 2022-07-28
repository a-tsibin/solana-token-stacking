use crate::{
    events::SellFctrTokensEvent,
    state::{Platform, User},
};
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, system_instruction},
};
use anchor_spl::token;
use anchor_spl::token::{Burn, Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct SellFctrTokens<'info> {
    #[account(seeds = [b"user", authority.key().as_ref()], bump = user.bump)]
    user: Account<'info, User>,
    #[account(mut, seeds = [b"fctr_vault", authority.key().as_ref()], bump = user.bump_fctr_vault)]
    fctr_vault: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"sol_vault"], bump = platform.bump_sol_vault)]
    sol_vault: AccountInfo<'info>,
    #[account(mut, seeds = [b"platform"], bump = platform.bump)]
    platform: Account<'info, Platform>,
    #[account(mut, seeds = [b"fctr_mint"], bump = platform.bump_fctr_mint)]
    fctr_mint: Account<'info, Mint>,
    #[account(mut, address = user.authority)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

pub fn sell_fctr_tokens(ctx: Context<SellFctrTokens>) -> Result<()> {
    let lamports_to_get = ctx.accounts.fctr_vault.amount * LAMPORTS_PER_SOL / 101;
    invoke_signed(
        &system_instruction::transfer(
            ctx.accounts.sol_vault.key,
            ctx.accounts.authority.key,
            lamports_to_get,
        ),
        &[
            ctx.accounts.sol_vault.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        ],
        &[&[b"sol_vault", &[ctx.accounts.platform.bump_sol_vault]]],
    )?;

    let signer: &[&[&[u8]]] = &[&[b"platform", &[ctx.accounts.platform.bump]]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.fctr_mint.to_account_info(),
            from: ctx.accounts.fctr_vault.to_account_info(),
            authority: ctx.accounts.platform.to_account_info(),
        },
        signer,
    );
    ctx.accounts.platform.fctr_token_total_amount -= ctx.accounts.fctr_vault.amount;
    token::burn(cpi_ctx, ctx.accounts.fctr_vault.amount)?;

    emit!(SellFctrTokensEvent {
        amount: ctx.accounts.fctr_vault.amount,
    });

    Ok(())
}
