use anchor_lang::prelude::*;

use crate::state::Streamer;
use crate::state::Offer;
use crate::state::Deal;

use crate::error::ErrorCode;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(id: u64, offer_id: u64)]
pub struct CreateDeal<'info> {
    #[account(
        init, 
        seeds = [DEAL_SEED.as_bytes(), id.to_le_bytes().as_ref()],
        bump,
        payer = owner, 
        space = 8 + Deal::INIT_SPACE,
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
        seeds = [STREAMER_SEED.as_bytes(), owner.key().as_ref()],
        bump,
    )]
    pub streamer: Account<'info, Streamer>,

    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_deal(
    ctx: Context<CreateDeal>,
    id: u64, 
    offer_id: u64, 
    encrypted_data: String
) -> Result<()> {
    require!(ctx.accounts.streamer.owner == *ctx.accounts.owner.key, ErrorCode::StreamerNotRegistered);

    let streamer = &mut ctx.accounts.streamer;
    let _offer = &mut ctx.accounts.offer;
    let deal = &mut ctx.accounts.deal;

    deal.id = id;
    deal.offer_id = offer_id;
    deal.streamer = streamer.key();
    deal.is_accepted = false;
    deal.is_completed = false;
    deal.encrypted_data = encrypted_data;

    msg!("Deal created...");
    Ok(())
}
