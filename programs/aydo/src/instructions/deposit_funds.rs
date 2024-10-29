use anchor_lang::prelude::*;

use crate::state::Buyer;
use crate::error::ErrorCode;
use crate::constants::*;

#[derive(Accounts)]
pub struct DepositFunds<'info> {
    #[account(mut)]
    pub buyer: Account<'info, Buyer>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_funds(
    ctx: Context<DepositFunds>, 
    amount: u64
) -> Result<()> {
    require!(ctx.accounts.buyer.owner == *ctx.accounts.owner.key, ErrorCode::BuyerNotRegistered);
    
    let amount2 = amount * LAMPORTS_PER_SOL;
    let buyer = &mut ctx.accounts.buyer;
    let owner = &ctx.accounts.owner;

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &owner.key(),
        &buyer.key(),
        amount2,
    );

    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            owner.to_account_info(),
            buyer.to_account_info(),
        ],
    )?;

    buyer.balance = buyer.balance.checked_add(amount2).ok_or(ErrorCode::BalanceOverflow)?;

    msg!("The deposit has been topped up...");
    Ok(())
}
