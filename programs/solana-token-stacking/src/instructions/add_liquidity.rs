use crate::{events::LiquidityAddedEvent, state::Platform};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    /// CHECK:
    #[account(mut, seeds = [b"sol_vault"], bump = platform.bump_sol_vault)]
    sol_vault: AccountInfo<'info>,
    #[account(mut, seeds = [b"platform"], bump = platform.bump)]
    platform: Account<'info, Platform>,
    #[account(mut, address = platform.authority)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn add_liquidity(ctx: Context<AddLiquidity>, amount: u64) -> Result<()> {
    invoke(
        &system_instruction::transfer(
            ctx.accounts.authority.key,
            ctx.accounts.sol_vault.key,
            amount,
        ),
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.sol_vault.to_account_info(),
        ],
    )?;

    emit!(LiquidityAddedEvent { amount });

    Ok(())
}
