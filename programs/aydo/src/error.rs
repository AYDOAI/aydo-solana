use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Buyer is not registered.")]
    BuyerNotRegistered,

    #[msg("Streamer is not registered.")]
    StreamerNotRegistered,

    #[msg("Balance overflow occurred.")]
    BalanceOverflow,

    #[msg("Buyer already exists.")]
    BuyerAlreadyExists,

    #[msg("Offer already exists.")]
    OfferAlreadyExists,

    #[msg("Offer is not active.")]
    OfferIsNotActive,

    #[msg("Deal is accepted.")]
    DealIsAccepted,
    
    #[msg("Deal is not accepted.")]
    DealIsNotAccepted,

    #[msg("Deal is completed.")]
    DealIsCompleted,

    #[msg("Request forbidden.")]
    RequestForbidden,

    #[msg("Not enough funds.")]
    FundsNotEnough,
}
