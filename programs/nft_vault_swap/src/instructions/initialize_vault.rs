use anchor_lang::prelude::*;
use crate::state::vault::Vault;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut, signer)]
    pub signer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

// pub fn handler(ctx: Context<InitializeVault>) -> Result<()> {
//     ctx.accounts.vault.bump = *ctx.bumps.get("vault").unwrap();
//     Ok(())
// }
