use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Whitelisted {
    pub address: Pubkey,
    pub bump: u8,
}