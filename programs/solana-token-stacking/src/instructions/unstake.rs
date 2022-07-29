use crate::state::Receipt;
use crate::{
    errors::CustomErrors,
    events::UnstakeEvent,
    state::{Platform, User},
};
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount, Transfer};
use itertools::Itertools;

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
    platform: Box<Account<'info, Platform>>,
    #[account(mut, seeds = [b"fctr_token_vault"], bump = platform.bump_fctr_token_vault)]
    platform_fctr_token_vault: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"bcdev_mint"], bump = platform.bump_bcdev_mint)]
    bcdev_mint: Account<'info, Mint>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    clock: Sysvar<'info, Clock>,
}

pub fn unstake<'info>(ctx: Context<'_, '_, '_, 'info, Unstake<'info>>) -> Result<()> {
    let now: u64 = ctx.accounts.clock.unix_timestamp as _;
    if ctx.accounts.receipt.authority != ctx.accounts.authority.key() {
        return err!(CustomErrors::InvalidReceiptAuthority);
    }
    if !ctx.accounts.receipt.is_valid {
        return err!(CustomErrors::InvalidReceipt);
    }
    if ctx.accounts.platform.round_start < ctx.accounts.receipt.stake_ts {
        return err!(CustomErrors::RoundStillGoing);
    }

    if ctx.remaining_accounts.len() / 3 != ctx.accounts.receipt.grantors.len() {
        return err!(CustomErrors::InvalidGrantorsList);
    }

    let grantors_accounts = ctx
        .remaining_accounts
        .chunks_exact(3)
        .take(12)
        .map(|pair| {
            let grantor_user = Account::<User>::try_from(&pair[0])?;
            let grantor_fctr_vault = Account::<TokenAccount>::try_from(&pair[1])?;
            let grantor_bcdev_vault = Account::<TokenAccount>::try_from(&pair[2])?;
            let grantor_from_account = ctx
                .accounts
                .receipt
                .grantors
                .iter()
                .find(|g| g.grantor == grantor_user.authority)
                .ok_or(CustomErrors::InvalidGrantorsList)?;
            let grantor = GrantorsToReward {
                user: grantor_user,
                fctr_vault: grantor_fctr_vault,
                bcdev_vault: grantor_bcdev_vault,
                grant_amount: grantor_from_account.amount,
                grant_duration: grantor_from_account.grant_duration,
            };
            ctx.accounts
                .receipt
                .grantors
                .retain(|g| g.grantor != grantor.user.authority);
            Ok(grantor)
        })
        .collect::<Result<Vec<GrantorsToReward>>>()?
        .into_iter()
        .sorted_by(|a, b| a.grant_duration.cmp(&b.grant_duration))
        .collect::<Vec<GrantorsToReward>>();

    if (ctx.accounts.receipt.grantors.len() != 0)
        || (grantors_accounts.len() != ctx.accounts.receipt.grantors.len())
    {
        return err!(CustomErrors::InvalidGrantorsList);
    }

    let total_reward = calculate_reward(&grantors_accounts, &ctx);
    let total_granted_fctr = return_fctr(&grantors_accounts, &ctx)?;
    mint_reward(&grantors_accounts, &ctx, total_reward, total_granted_fctr)?;

    ctx.accounts.receipt.is_valid = false;
    ctx.accounts.receipt.grantors.clear();
    ctx.accounts
        .receipt
        .grantors_history
        .retain(|g| (g.grant_ts + 30 * ctx.accounts.platform.round_duration) < now);

    emit!(UnstakeEvent {});

    Ok(())
}

fn calculate_reward<'info>(
    grantors_to_reward: &Vec<GrantorsToReward>,
    ctx: &Context<'_, '_, '_, 'info, Unstake<'info>>,
) -> u64 {
    let apr = ctx.accounts.receipt.apr + grantors_to_reward.len() as f64;
    let reward =
        grantors_to_reward
            .iter()
            .fold(AprReward::apply(apr, 0), |reward_amount, grantor| {
                let reward_percentage = apr
                    * (grantor.grant_duration as f64 / ctx.accounts.platform.round_duration as f64);
                let g_reward = (ctx.accounts.receipt.amount_deposited as f64 * reward_percentage)
                    .round() as u64;
                AprReward::apply(reward_amount.apr - 0.02, reward_amount.reward + g_reward)
            });
    reward.reward
}

fn return_fctr<'info>(
    grantors_to_reward: &Vec<GrantorsToReward<'info>>,
    ctx: &Context<'_, '_, '_, 'info, Unstake<'info>>,
) -> Result<u64> {
    let signer: &[&[&[u8]]] = &[&[b"platform", &[ctx.accounts.platform.bump]]];
    let total_ftcr_granted = grantors_to_reward
        .iter()
        .fold(Ok(0), |sum: Result<u64>, g| {
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.platform_fctr_token_vault.to_account_info(),
                    to: g.fctr_vault.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
                signer,
            );
            token::transfer(cpi_ctx, g.grant_amount)?;
            sum.map(|s| s + g.grant_amount)
        })?;

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.platform_fctr_token_vault.to_account_info(),
            to: ctx.accounts.fctr_vault.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
        signer,
    );
    token::transfer(
        cpi_ctx,
        ctx.accounts.receipt.amount_deposited - total_ftcr_granted,
    )?;
    Ok(total_ftcr_granted)
}

fn mint_reward<'info>(
    grantors_to_reward: &Vec<GrantorsToReward<'info>>,
    ctx: &Context<'_, '_, '_, 'info, Unstake<'info>>,
    total_reward: u64,
    total_granted_fctr: u64,
) -> Result<()> {
    let signer: &[&[&[u8]]] = &[&[b"platform", &[ctx.accounts.platform.bump]]];
    if grantors_to_reward.len() != 0 {
        let grantors_reward = total_reward - total_reward / 2;
        let staker_reward = total_reward - grantors_reward;
        mint_bcdev(staker_reward, &ctx.accounts.bcdev_vault, ctx, signer)?;

        grantors_to_reward
            .iter()
            .map(|g| {
                let share = grantors_reward * g.grant_amount / total_granted_fctr;
                mint_bcdev(share, &g.bcdev_vault, ctx, signer)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(())
    } else {
        mint_bcdev(total_reward, &ctx.accounts.bcdev_vault, ctx, signer)
    }
}

fn mint_bcdev<'info>(
    amount: u64,
    bcdev_vault: &Account<'info, TokenAccount>,
    ctx: &Context<'_, '_, '_, 'info, Unstake<'info>>,
    signer: &[&[&[u8]]],
) -> Result<()> {
    let mint_cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.bcdev_mint.to_account_info(),
            to: bcdev_vault.to_account_info(),
            authority: ctx.accounts.platform.to_account_info(),
        },
        signer,
    );
    token::mint_to(mint_cpi_ctx, amount)
}

struct GrantorsToReward<'a> {
    pub user: Account<'a, User>,
    pub fctr_vault: Account<'a, TokenAccount>,
    pub bcdev_vault: Account<'a, TokenAccount>,
    pub grant_amount: u64,
    pub grant_duration: u64,
}

struct AprReward {
    pub apr: f64,
    pub reward: u64,
}

impl AprReward {
    pub fn apply(apr: f64, reward: u64) -> AprReward {
        AprReward { apr, reward }
    }
}
