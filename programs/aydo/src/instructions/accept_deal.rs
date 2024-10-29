use anchor_lang::prelude::*;

use crate::state::Buyer;
use crate::state::Offer;
use crate::state::Deal;

use crate::error::ErrorCode;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(id: u64, offer_id: u64)]
pub struct AcceptDeal<'info> {
    #[account(
        mut, 
        seeds = [DEAL_SEED.as_bytes(), id.to_le_bytes().as_ref()],
        bump,
    )]
    pub deal: Account<'info, Deal>,

    #[account(
        mut,
        seeds = [OFFER_SEED.as_bytes(), offer_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        seeds = [BUYER_SEED.as_bytes(), owner.key().as_ref()],
        bump,
    )]
    pub buyer: Account<'info, Buyer>,

    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn accept_deal(
    ctx: Context<AcceptDeal>,
    _id: u64, 
    _offer_id: u64
) -> Result<()> {
    require!(ctx.accounts.buyer.owner == *ctx.accounts.owner.key, ErrorCode::RequestForbidden);
    require!(ctx.accounts.offer.buyer == ctx.accounts.buyer.key(), ErrorCode::RequestForbidden);
    require!(ctx.accounts.offer.id == ctx.accounts.deal.offer_id, ErrorCode::RequestForbidden);

    require!(ctx.accounts.offer.is_active == true, ErrorCode::OfferIsNotActive);
    require!(ctx.accounts.deal.is_accepted == false, ErrorCode::DealIsAccepted);
    require!(ctx.accounts.deal.is_completed == false, ErrorCode::DealIsCompleted);

    let _buyer = &mut ctx.accounts.buyer;
    let _offer = &mut ctx.accounts.offer;
    let deal = &mut ctx.accounts.deal;

    deal.is_accepted = true;

    msg!("Deal accepted...");
    Ok(())
}
