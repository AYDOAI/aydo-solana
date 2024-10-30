use anchor_lang::prelude::*;

use crate::state::Buyer;
use crate::state::Offer;
use crate::error::ErrorCode;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(id : u64)]
pub struct CreateOffer<'info> {
    #[account(
        init, 
        seeds = [OFFER_SEED.as_bytes(), id.to_le_bytes().as_ref()],
        bump,
        payer = owner, 
        space = 8 + Offer::INIT_SPACE,
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

pub fn create_offer(
    ctx: Context<CreateOffer>,
    id: u64, 
    location: String, 
    price: u64
) -> Result<()> {
    require!(ctx.accounts.buyer.owner == *ctx.accounts.owner.key, ErrorCode::BuyerNotRegistered);

    let buyer = &ctx.accounts.buyer;
    let offer = &mut ctx.accounts.offer;

    offer.buyer = buyer.key();
    offer.is_active = true;
    offer.id = id;
    offer.location = location;
    offer.price = price;

    msg!("Offer created...");
    Ok(())
}
