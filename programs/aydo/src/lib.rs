use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

pub mod error;
pub mod constants;
pub mod instructions;
pub mod state;

declare_id!("11111111111111111111111111111111");


#[program]
mod aydo {
    use super::*;

    pub fn create_buyer(ctx: Context<CreateBuyer>) -> Result<()> {
        create_buyer::create_buyer(ctx)
    }

    pub fn create_streamer(ctx: Context<CreateStreamer>) -> Result<()> {
        create_streamer::create_streamer(ctx)
    }

    pub fn deposit_funds(ctx: Context<DepositFunds>, amount: u64) -> Result<()> {
        deposit_funds::deposit_funds(ctx, amount)
    }

    pub fn create_offer(ctx: Context<CreateOffer>, id: u64, location: String, price: u64) -> Result<()> {
        create_offer::create_offer(ctx, id, location, price)
    }

    pub fn delete_offer(ctx: Context<DeleteOffer>, id: u64) -> Result<()> {
        delete_offer::delete_offer(ctx, id)
    }

    pub fn create_deal(ctx: Context<CreateDeal>, id: u64, offer_id: u64, encrypted_data: String) -> Result<()> {
        create_deal::create_deal(ctx, id, offer_id, encrypted_data)
    }
}
