use crate::state::Deal;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(id : u64)]
pub struct CreateDeal<'info> {
    #[account(
        init, 
        seeds = [b"deal", id.to_le_bytes().as_ref()],
        bump,
        payer = owner, 
        space = 8 + Deal::INIT_SPACE,
    )]
    pub deal: Account<'info, Deal>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
