use anchor_lang::{
    prelude::*, 
    system_program
};

use crate::WhitelistError;
use crate::Whitelisted;

#[derive(Accounts)]
#[instruction(address:Pubkey)]
pub struct WhitelistOperations<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        space= 8 + Whitelisted::INIT_SPACE,
        seeds = [b"whitelisted", address.as_ref()],
        bump,
    )]
    pub whitelisted: Account<'info, Whitelisted>,
    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistOperations<'info> {
    pub fn whitelist(
        &mut self, 
        address: Pubkey,
        bump: u8
    ) -> Result<()> {
        if self.whitelisted.address == address {
            return err!(WhitelistError::AddressAlreadyWhitelisted);
        }

        self.whitelisted.set_inner(Whitelisted {
            address,
            bump,
        });
        msg!("Added {} to whitelist", address);
        Ok(())
    }

    pub fn unwhitelist(
        &mut self, 
        address: Pubkey
    ) -> Result<()> {
        if self.whitelisted.address != address {
            return err!(WhitelistError::NotWhitelisted);
        }

        self.whitelisted.close(self.admin.to_account_info())?;
        msg!("Removed {} from whitelist", address);
        Ok(())
    }
}
