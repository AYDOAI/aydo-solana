use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};
use crate::state::Buyer;
use crate::error::ErrorCode;
use crate::constants::*;
use std::str::FromStr;


#[derive(Accounts)]
pub struct DepositFunds<'info> {
    #[account(
        mut,
        seeds = [BUYER_SEED.as_bytes(), owner.key().as_ref()],
        bump,
    )]
    pub buyer: Account<'info, Buyer>,
    
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        constraint = owner_token_account.owner == *owner.key @ ErrorCode::InvalidTokenAccount,
        constraint = owner_token_account.mint == Pubkey::from_str(AYDO_MINT_PUBKEY).unwrap() @ ErrorCode::InvalidTokenMint,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = buyer_token_account.owner == buyer.key() @ ErrorCode::InvalidTokenAccount,
        constraint = buyer_token_account.mint == Pubkey::from_str(AYDO_MINT_PUBKEY).unwrap() @ ErrorCode::InvalidTokenMint,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

pub fn deposit_funds(
    ctx: Context<DepositFunds>, 
    amount: u64
) -> Result<()> {
    require!(ctx.accounts.buyer.owner == *ctx.accounts.owner.key, ErrorCode::BuyerNotRegistered);
    
    let buyer = &mut ctx.accounts.buyer;
    let cpi_accounts = Transfer {
        from: ctx.accounts.owner_token_account.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    
    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_context, amount)?;
    
    buyer.balance = buyer.balance.checked_add(amount).ok_or(ErrorCode::BalanceOverflow)?;
    
    msg!("The deposit has been topped up with tokens...");
    Ok(())
}
