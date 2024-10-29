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

    #[msg("Request forbidden.")]
    RequestForbidden,
}
