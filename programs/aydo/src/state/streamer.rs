use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Streamer {
    pub owner: Pubkey,
    pub balance: u64,
}
