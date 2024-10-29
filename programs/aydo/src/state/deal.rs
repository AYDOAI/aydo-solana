use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Deal {
    pub id: u64,
    pub offer: Pubkey,
    pub streamer: Pubkey,
    pub is_accepted: bool,
    pub is_completed: bool,
    #[max_len(256)]
    pub encrypted_data: String,
}
