use anchor_lang::prelude::*;

use crate::state::Streamer;
use crate::constants::*;

#[derive(Accounts)]
pub struct CreateStreamer<'info> {
    #[account(
        init, 
        seeds = [STREAMER_SEED.as_bytes(), owner.key().as_ref()],
        bump,
        payer = owner, 
        space = 8 + Streamer::INIT_SPACE,
    )]
    pub streamer: Account<'info, Streamer>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_streamer(
    ctx: Context<CreateStreamer>,
) -> Result<()> {
    let streamer = &mut ctx.accounts.streamer;
    streamer.owner = *ctx.accounts.owner.key;
    streamer.balance = 0;

    msg!("Streamer created...");
    Ok(())
}
