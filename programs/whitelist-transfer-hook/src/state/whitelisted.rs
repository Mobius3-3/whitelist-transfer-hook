use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Whitelisted {
    pub bump: u8,
}