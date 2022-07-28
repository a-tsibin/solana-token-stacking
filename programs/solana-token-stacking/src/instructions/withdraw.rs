use crate::{errors::CustomErrors, events::WithdrawEvent, state::Platform};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, system_instruction},
};
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"sol_vault"], bump = platform.bump_sol_vault)]
    sol_vault: AccountInfo<'info>,
    #[account(mut, seeds = [b"platform"], bump = platform.bump)]
    platform: Account<'info, Platform>,
    #[account(mut, address = platform.authority)]
    authority: Signer<'info>,
    #[account(
        seeds = [b"fctr_token_vault"],
        bump = platform.bump_fctr_token_vault
    )]
    fctr_token_vault: Account<'info, TokenAccount>,
    clock: Sysvar<'info, Clock>,
    system_program: Program<'info, System>,
}

pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    let now = ctx.accounts.clock.unix_timestamp as _;
    if !check_withdraw_conditions(&ctx.accounts, now) {
        return err!(CustomErrors::WithdrawConditions);
    }

    invoke_signed(
        &system_instruction::transfer(
            ctx.accounts.sol_vault.key,
            ctx.accounts.authority.key,
            ctx.accounts.sol_vault.lamports() - Rent::get()?.minimum_balance(0),
        ),
        &[
            ctx.accounts.sol_vault.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        ],
        &[&[b"sol_vault", &[ctx.accounts.platform.bump_sol_vault]]],
    )?;

    emit!(WithdrawEvent {});

    Ok(())
}

fn check_withdraw_conditions(accounts: &Withdraw, now: u64) -> bool {
    if (accounts.platform.fctr_token_total_amount == accounts.fctr_token_vault.amount
        && accounts.platform.bcdev_token_total_amount == 0)
        || (accounts.platform.is_final
            && now > accounts.platform.round_start + 3 * accounts.platform.round_duration)
    {
        return true;
    }
    return false;
}
