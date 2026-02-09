use anchor_lang::prelude::*;

use crate::state::whitelisted::Whitelisted;

#[derive(Accounts)]
#[instruction(address: Pubkey)]
pub struct Whitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = Whitelisted::DISCRIMINATOR.len() + Whitelisted::INIT_SPACE,
        seeds = [b"whitelisted", address.key().as_ref()],
        bump,
    )]
    pub whitelisted: Account<'info, Whitelisted>,
    pub system_program: Program<'info, System>,
}

impl<'info> Whitelist<'info> {
    pub fn whitelist(
        &mut self,
        _address: Pubkey,
        bump: u8
    ) -> Result<()> {
        self.whitelisted.set_inner(Whitelisted {
            bump,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(address: Pubkey)]
pub struct Unwhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        close = admin,
        seeds = [b"whitelisted", address.key().as_ref()],
        bump = whitelisted.bump,
    )]
    pub whitelisted: Account<'info, Whitelisted>,
    pub system_program: Program<'info, System>,
}

impl<'info> Unwhitelist<'info> {
    pub fn unwhitelist(&mut self, _address: Pubkey) -> Result<()> {
        Ok(())
    }
}
