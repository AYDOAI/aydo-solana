use anchor_lang::prelude::*;
use std::str::FromStr;

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

    #[account(mut)]
    pub commission_owner: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}


pub fn withdraw_reward(
    ctx: Context<WithdrawReward>,
    amount: u64
) -> Result<()> {
    require!(ctx.accounts.streamer.owner == *ctx.accounts.owner.key, ErrorCode::RequestForbidden);

    let commission_address: Pubkey = Pubkey::from_str(AYDO_COMMISSION_ADDRESS).unwrap();
    require!(ctx.accounts.commission_owner.key() == commission_address, ErrorCode::Unauthorized);

    let owner = &mut ctx.accounts.owner;
    let streamer = &mut ctx.accounts.streamer;
    let commission_owner = &mut ctx.accounts.commission_owner;
    let balance = streamer.to_account_info().lamports();
    require!(amount <= balance, ErrorCode::FundsNotEnough);

    let commission = amount.checked_mul(AYDO_COMMISSION).ok_or(ErrorCode::Overflow)? / 100;
    let reward = amount.checked_sub(commission).ok_or(ErrorCode::Overflow)?;

    **streamer.to_account_info().try_borrow_mut_lamports()? -= amount;
    **owner.to_account_info().try_borrow_mut_lamports()? += reward;
    **commission_owner.to_account_info().try_borrow_mut_lamports()? += commission;

    streamer.balance = streamer.balance.checked_sub(amount).ok_or(ErrorCode::BalanceOverflow)?;

    msg!("Withdraw is completed...");
    Ok(())
}
