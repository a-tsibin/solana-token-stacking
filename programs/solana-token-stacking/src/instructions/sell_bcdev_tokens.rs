use crate::{
    events::SellBcdevTokensEvent,
    state::{Platform, User},
    BCDEV_DECIMALS,
};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, system_instruction},
};
use anchor_spl::token;
use anchor_spl::token::spl_token::native_mint::DECIMALS;
use anchor_spl::token::{Burn, Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct SellBcdevTokens<'info> {
    #[account(seeds = [b"user", authority.key().as_ref()], bump = user.bump)]
    user: Account<'info, User>,
    #[account(mut, seeds = [b"bcdev_vault", authority.key().as_ref()], bump = user.bump_bcdev_vault)]
    bcdev_vault: Account<'info, TokenAccount>,
    /// CHECK:
    #[account(mut, seeds = [b"sol_vault"], bump = platform.bump_sol_vault)]
    sol_vault: AccountInfo<'info>,
    #[account(mut, seeds = [b"platform"], bump = platform.bump)]
    platform: Account<'info, Platform>,
    #[account(mut, seeds = [b"bcdev_mint"], bump = platform.bump_bcdev_mint)]
    bcdev_mint: Account<'info, Mint>,
    #[account(mut, address = user.authority)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

pub fn sell_bcdev_tokens(ctx: Context<SellBcdevTokens>, amount: u64) -> Result<()> {
    let lamports_to_get =
        ctx.accounts.bcdev_vault.amount / (11 * 10u64.pow((BCDEV_DECIMALS - DECIMALS) as _));
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
            mint: ctx.accounts.bcdev_mint.to_account_info(),
            from: ctx.accounts.bcdev_vault.to_account_info(),
            authority: ctx.accounts.platform.to_account_info(),
        },
        signer,
    );
    ctx.accounts.platform.bcdev_token_total_amount -= amount;
    token::burn(cpi_ctx, amount)?;

    emit!(SellBcdevTokensEvent { amount });

    Ok(())
}
