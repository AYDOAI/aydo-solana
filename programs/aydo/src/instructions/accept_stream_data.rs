use anchor_lang::prelude::*;

use crate::state::Buyer;
use crate::state::Streamer;
use crate::state::Offer;
use crate::state::Deal;

use crate::error::ErrorCode;
use crate::constants::*;

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

    let _deal = &mut ctx.accounts.deal;
    let offer = &mut ctx.accounts.offer;
    let buyer = &mut ctx.accounts.buyer;
    let streamer = &mut ctx.accounts.streamer;

    let amount = offer.price * count as u64;
    let buyer_balance = buyer.to_account_info().lamports();
    require!(amount <= buyer_balance, ErrorCode::FundsNotEnough);

    **buyer.to_account_info().try_borrow_mut_lamports()? -= amount;
    **streamer.to_account_info().try_borrow_mut_lamports()? += amount;

    buyer.balance = buyer.balance.checked_sub(amount).ok_or(ErrorCode::BalanceOverflow)?;
    streamer.balance = streamer.balance.checked_add(amount).ok_or(ErrorCode::BalanceOverflow)?;

    msg!("Stream data accepted...");

    Ok(())
}
