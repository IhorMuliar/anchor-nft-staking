use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError {
    #[msg("Incorrect mint")]
    IncorrectMint,
    #[msg("Incorrect collection")]
    IncorrectCollection,
    #[msg("Collection not verified")]
    CollectionNotVerified,
    #[msg("Max stake reached")]
    MaxStakeReached,
    #[msg("Unstake delay not met")]
    UnstakeDelayNotMet,
    #[msg("Unauthorized - not the owner")]
    Unauthorized,
    #[msg("No points to claim")]
    NoPointsToClaim,
    #[msg("Invalid configuration")]
    InvalidConfiguration,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}