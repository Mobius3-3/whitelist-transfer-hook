#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

mod instructions;
mod state;
mod error;

pub use error::*;
pub use instructions::*;
pub use state::*;

use spl_discriminator::SplDiscriminate;
use spl_transfer_hook_interface::{
    instruction::{
        ExecuteInstruction, 
        InitializeExtraAccountMetaListInstruction
    },
};
use spl_tlv_account_resolution::state::ExtraAccountMetaList;

declare_id!("DhzyDgCmmQzVC4vEcj2zRGUyN8Mt5JynfdGLKkBcRGaX");

#[program]
pub mod whitelist_transfer_hook {
    use super::*;

    pub fn init_mint(ctx: Context<TokenFactory>) -> Result<()> {
        ctx.accounts.init_mint()
    }

    pub fn whitelist(ctx: Context<Whitelist>, address: Pubkey) -> Result<()> {
        let bump = ctx.bumps.whitelisted;
        ctx.accounts.whitelist(address, bump)
    }

    pub fn unwhitelist(ctx: Context<Unwhitelist>, address: Pubkey) -> Result<()> {
        ctx.accounts.unwhitelist(address)
    }

    pub fn initialize_transfer_hook(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {

        msg!("Initializing Transfer Hook...");

        // Get the extra account metas for the transfer hook
        let extra_account_metas = InitializeExtraAccountMetaList::extra_account_metas()?;

        msg!("Extra Account Metas: {:?}", extra_account_metas);
        msg!("Extra Account Metas Length: {}", extra_account_metas.len());

        // initialize ExtraAccountMetaList account with extra accounts
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas
        ).unwrap();

        Ok(())
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        // Call the transfer hook logic
        ctx.accounts.transfer_hook(amount)
    }
}
