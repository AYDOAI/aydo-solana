use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub buyer: Pubkey,
    pub is_active: bool,
    #[max_len(64)]
    pub location: String,
    pub price: u64,
}
