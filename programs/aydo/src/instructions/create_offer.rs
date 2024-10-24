use crate::state::Offer;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(id : u64)]
pub struct CreateOffer<'info> {
    #[account(
        init, 
        seeds = [b"offer", id.to_le_bytes().as_ref()],
        bump,
        payer = owner, 
        space = 8 + Offer::INIT_SPACE,
    )]
    pub offer: Account<'info, Offer>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
