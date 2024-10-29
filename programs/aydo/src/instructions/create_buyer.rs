use anchor_lang::prelude::*;

use crate::state::Buyer;
use crate::constants::*;

#[derive(Accounts)]
pub struct CreateBuyer<'info> {
    #[account(
        init, 
        seeds = [BUYER_SEED.as_bytes(), owner.key().as_ref()],
        bump,
        payer = owner, 
        space = 8 + Buyer::INIT_SPACE,
    )]
    pub buyer: Account<'info, Buyer>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_buyer(
    ctx: Context<CreateBuyer>,
) -> Result<()> {
    let buyer = &mut ctx.accounts.buyer;
    buyer.owner = *ctx.accounts.owner.key;
    buyer.balance = 0;

    msg!("Buyer created...");
    Ok(())
}
