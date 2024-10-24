use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Buyer {
    pub owner: Pubkey,
    pub balance: u64,
}
