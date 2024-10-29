use anchor_lang::prelude::*;

use crate::state::Buyer;
use crate::state::Offer;
use crate::error::ErrorCode;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(id : u64)]
pub struct DeleteOffer<'info> {
    #[account(
        mut,
        seeds = [OFFER_SEED.as_bytes(), id.to_le_bytes().as_ref()],
        bump,
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        seeds = [BUYER_SEED.as_bytes(), owner.key().as_ref()],
        bump,
    )]
    pub buyer: Account<'info, Buyer>,

    pub owner: Signer<'info>,
}

pub fn delete_offer(
    ctx: Context<DeleteOffer>,
    _id: u64, 
) -> Result<()> {
    require!(ctx.accounts.buyer.owner == *ctx.accounts.owner.key, ErrorCode::BuyerNotRegistered);

    let _buyer = &mut ctx.accounts.buyer;
    let offer = &mut ctx.accounts.offer;

    offer.is_active = false;

    msg!("Offer deleted...");
    Ok(())
}
