use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, TransferChecked, Token2022};
use anchor_spl::token_interface::{TokenAccount, Mint};
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

    #[account(
        mut,
        constraint = commission_owner.key() == Pubkey::from_str(AYDO_COMMISSION_ADDRESS).unwrap() @ ErrorCode::Unauthorized
    )]
    pub commission_owner: AccountInfo<'info>,

    #[account(
        mut,
        constraint = streamer_token_account.owner == streamer.key() @ ErrorCode::InvalidTokenAccount,
        constraint = streamer_token_account.mint == mint.key() @ ErrorCode::InvalidTokenMint,
    )]
    pub streamer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = owner_token_account.owner == owner.key() @ ErrorCode::InvalidTokenAccount,
        constraint = owner_token_account.mint == mint.key() @ ErrorCode::InvalidTokenMint,
    )]
    pub owner_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = commission_token_account.owner == commission_owner.key() @ ErrorCode::InvalidTokenAccount,
        constraint = commission_token_account.mint == mint.key() @ ErrorCode::InvalidTokenMint,
    )]
    pub commission_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        constraint = mint.key() == Pubkey::from_str(AYDO_MINT_PUBKEY).unwrap() @ ErrorCode::InvalidTokenMint
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_reward(
    ctx: Context<WithdrawReward>,
    amount: u64
) -> Result<()> {
    require!(ctx.accounts.streamer.owner == *ctx.accounts.owner.key, ErrorCode::RequestForbidden);

    require!(
        ctx.accounts.mint.key() == Pubkey::from_str(AYDO_MINT_PUBKEY).unwrap(),
        ErrorCode::InvalidTokenMint
    );

    let _owner = &mut ctx.accounts.owner;
    let streamer = &mut ctx.accounts.streamer;
    let _commission_owner = &ctx.accounts.commission_owner;

    let streamer_balance = ctx.accounts.streamer_token_account.amount;
    require!(amount <= streamer_balance, ErrorCode::FundsNotEnough);

    let commission = amount.checked_mul(AYDO_COMMISSION).ok_or(ErrorCode::Overflow)? / 100;
    let reward = amount.checked_sub(commission).ok_or(ErrorCode::Overflow)?;

    let commission_transfer = TransferChecked {
        from: ctx.accounts.streamer_token_account.to_account_info(),
        to: ctx.accounts.commission_token_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };

    let commission_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), commission_transfer);
    token_2022::transfer_checked(commission_ctx, commission, ctx.accounts.mint.decimals)?;

    let reward_transfer = TransferChecked {
        from: ctx.accounts.streamer_token_account.to_account_info(),
        to: ctx.accounts.owner_token_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };

    let reward_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), reward_transfer);
    token_2022::transfer_checked(reward_ctx, reward, ctx.accounts.mint.decimals)?;

    streamer.balance = streamer.balance.checked_sub(amount).ok_or(ErrorCode::BalanceOverflow)?;

    msg!(
        "Withdraw completed: {} tokens sent to owner, {} tokens sent to commission.",
        reward,
        commission
    );

    Ok(())
}