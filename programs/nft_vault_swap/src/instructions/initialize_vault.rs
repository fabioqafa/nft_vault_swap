use anchor_lang::prelude::*;
use crate::state::vault::Vault;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = signer, seeds = [b"vault"], bump, space = 8 + 8)] //make sure that vault is unique
    pub vault: Account<'info, Vault>,
    #[account(mut, signer)]
    pub signer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeVault>, amount: u64) -> Result<()> {
    ctx.accounts.vault.owner = *ctx.accounts.signer.key; //here we assign the owner of the vault
    ctx.accounts.vault.price = amount; //the price of the NFT
    Ok(())
}

//write a function to withdraw everything from the vault, keep at least some rent to the program for the vault to not be deleted by the system
