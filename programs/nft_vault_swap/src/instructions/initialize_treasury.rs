use anchor_lang::prelude::*;
use crate::state::treasury::Treasury;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(init, payer = signer, seeds = [b"treasury"], bump, space = 8 + Treasury::INIT_SPACE)]
    //make sure that treasury is unique
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeTreasury>, rent: u64) -> Result<()> {
    ctx.accounts.treasury.owner = *ctx.accounts.signer.key;
    ctx.accounts.treasury.rent = rent;
    Ok(())
}

//write a function to withdraw everything from the vault, keep at least some rent to the program for the vault to not be deleted by the system
