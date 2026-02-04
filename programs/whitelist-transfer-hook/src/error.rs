use anchor_lang::prelude::*;

#[error_code]
pub enum WhitelistError {
    #[msg("This account is not whitelisted")]
    NotWhitelisted,
    #[msg("This account is already whitelisted")]
    AddressAlreadyWhitelisted,
}
