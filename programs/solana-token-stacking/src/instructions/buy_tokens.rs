use crate::{
    errors::CustomErrors,
    events::BuyFctrTokensEvent,
    state::{Platform, User},
};
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};
use anchor_spl::token;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount};

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(seeds = [b"user", authority.key().as_ref()], bump = user.bump)]
    user: Account<'info, User>,
    #[account(mut, seeds = [b"fctr_vault", authority.key().as_ref()], bump = user.bump_fctr_vault)]
    fctr_vault: Account<'info, TokenAccount>,
    /// CHECK:
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

pub fn buy_tokens(ctx: Context<BuyTokens>, fctr_to_buy: u64) -> Result<()> {
    if fctr_to_buy < 10 {
        return err!(CustomErrors::InvalidBuyAmount);
    }
    let lamports_to_spend = fctr_to_buy * LAMPORTS_PER_SOL / 109;
    invoke(
        &system_instruction::transfer(
            ctx.accounts.authority.key,
            ctx.accounts.sol_vault.key,
            lamports_to_spend,
        ),
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.sol_vault.to_account_info(),
        ],
    )?;
    ctx.accounts.user.total_fctr_amount += fctr_to_buy;
    ctx.accounts.platform.fctr_token_total_amount += fctr_to_buy;
    let signer: &[&[&[u8]]] = &[&[b"platform", &[ctx.accounts.platform.bump]]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.fctr_mint.to_account_info(),
            to: ctx.accounts.fctr_vault.to_account_info(),
            authority: ctx.accounts.platform.to_account_info(),
        },
        signer,
    );
    token::mint_to(cpi_ctx, fctr_to_buy)?;

    emit!(BuyFctrTokensEvent {
        amount: fctr_to_buy,
    });

    Ok(())
}
