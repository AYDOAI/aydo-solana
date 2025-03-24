use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, TransferChecked, Token2022};
use anchor_spl::token_interface::{TokenAccount, Mint};

use crate::state::Buyer;
use crate::state::Streamer;
use crate::state::Offer;
use crate::state::Deal;

use crate::error::ErrorCode;
use crate::constants::*;
use std::str::FromStr;

#[derive(Accounts)]
#[instruction(deal_id: u64)]
pub struct AcceptStreamData<'info> {
    #[account(
        mut,
        seeds = [BUYER_SEED.as_bytes(), owner.key().as_ref()],
        bump,
    )]
    pub buyer: Account<'info, Buyer>,

    #[account(
        mut, 
        has_one = offer,
        has_one = streamer,
        seeds = [DEAL_SEED.as_bytes(), deal_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub deal: Account<'info, Deal>,

    #[account(mut)]
    pub offer: Account<'info, Offer>,

    #[account(mut)]
    pub streamer: Account<'info, Streamer>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        constraint = buyer_token_account.owner == buyer.key() @ ErrorCode::InvalidTokenAccount,
        constraint = buyer_token_account.mint == mint.key() @ ErrorCode::InvalidTokenMint,
    )]
    pub buyer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = streamer_token_account.owner == streamer.key() @ ErrorCode::InvalidTokenAccount,
        constraint = streamer_token_account.mint == mint.key() @ ErrorCode::InvalidTokenMint,
    )]
    pub streamer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        constraint = mint.key() == Pubkey::from_str(AYDO_MINT_PUBKEY).unwrap() @ ErrorCode::InvalidTokenMint
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,

    pub system_program: Program<'info, System>,
}

pub fn accept_stream_data(
    ctx: Context<AcceptStreamData>,
    _deal_id: u64,
    count: u32
) -> Result<()> {
    require!(ctx.accounts.buyer.owner == *ctx.accounts.owner.key, ErrorCode::RequestForbidden);
    require!(ctx.accounts.offer.buyer == ctx.accounts.buyer.key(), ErrorCode::RequestForbidden);
    require!(ctx.accounts.deal.offer == ctx.accounts.offer.key(), ErrorCode::RequestForbidden);
    require!(ctx.accounts.deal.streamer == ctx.accounts.streamer.key(), ErrorCode::RequestForbidden);

    require!(ctx.accounts.offer.is_active == true, ErrorCode::OfferIsNotActive);
    require!(ctx.accounts.deal.is_accepted == true, ErrorCode::DealIsNotAccepted);
    require!(ctx.accounts.deal.is_completed == false, ErrorCode::DealIsCompleted);

    let offer = &ctx.accounts.offer;
    let buyer = &mut ctx.accounts.buyer;
    let streamer = &mut ctx.accounts.streamer;

    let amount = offer.price * count as u64;

    require!(
        ctx.accounts.mint.key() == Pubkey::from_str(AYDO_MINT_PUBKEY).unwrap(),
        ErrorCode::InvalidTokenMint
    );

    let buyer_balance = ctx.accounts.buyer_token_account.amount;
    require!(amount <= buyer_balance, ErrorCode::FundsNotEnough);

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.buyer_token_account.to_account_info(),
        to: ctx.accounts.streamer_token_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        cpi_accounts
    );
    token_2022::transfer_checked(cpi_context, amount, ctx.accounts.mint.decimals)?;

    buyer.balance = buyer.balance.checked_sub(amount).ok_or(ErrorCode::BalanceOverflow)?;
    streamer.balance = streamer.balance.checked_add(amount).ok_or(ErrorCode::BalanceOverflow)?;

    msg!("Stream data accepted, tokens transferred...");

    Ok(())
}