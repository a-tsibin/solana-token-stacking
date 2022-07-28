use crate::{errors::CustomErrors, events::RoundStartEvent, state::Platform};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct StartRound<'info> {
    #[account(mut, seeds = [b"platform"], bump = platform.bump)]
    platform: Account<'info, Platform>,
    #[account(mut, address = platform.authority)]
    authority: Signer<'info>,
    clock: Sysvar<'info, Clock>,
    system_program: Program<'info, System>,
}

pub fn start_round(ctx: Context<StartRound>, is_final: bool) -> Result<()> {
    let now = ctx.accounts.clock.unix_timestamp as _;
    if now < ctx.accounts.platform.round_start + ctx.accounts.platform.round_duration {
        return err!(CustomErrors::RoundAlreadyStarted);
    } else if ctx.accounts.platform.is_final {
        return err!(CustomErrors::StackingFinished);
    }
    ctx.accounts.platform.round_start = now;
    ctx.accounts.platform.is_final = is_final;

    emit!(RoundStartEvent { is_final });

    Ok(())
}
