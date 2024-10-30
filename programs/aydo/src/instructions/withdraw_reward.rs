use anchor_lang::prelude::*;

use crate::state::Streamer;

use crate::error::ErrorCode;
use crate::constants::*;

#[derive(Accounts)]
pub struct WithdrawReward<'info> {
    #[account(
        mut,
        seeds = [STREAMER_SEED.as_bytes(), owner.key().as_ref()],
        bump,
    )]
    pub streamer: Account<'info, Streamer>,

    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}


pub fn withdraw_reward(
    ctx: Context<WithdrawReward>,
    amount: u64
) -> Result<()> {
    require!(ctx.accounts.streamer.owner == *ctx.accounts.owner.key, ErrorCode::RequestForbidden);

    let owner = &mut ctx.accounts.owner;
    let streamer = &mut ctx.accounts.streamer;
    let balance = streamer.to_account_info().lamports();
    require!(amount <= balance, ErrorCode::FundsNotEnough);

    **streamer.to_account_info().try_borrow_mut_lamports()? -= amount;
    **owner.to_account_info().try_borrow_mut_lamports()? += amount;

    streamer.balance = streamer.balance.checked_sub(amount).ok_or(ErrorCode::BalanceOverflow)?;

    msg!("Withdraw is completed...");

    Ok(())
}
