use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, Mint, TokenAccount, Transfer},
    associated_token::AssociatedToken
};
use anchor_spl::token;
use crate::state::vault::Vault;

#[derive(Accounts)]
pub struct LockNFT<'info> {
    #[account(mut, signer)]
    pub signer: AccountInfo<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = vault,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<LockNFT>) -> Result<()> {
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        },
    );

    token::transfer(cpi_context, 1)?;

    // Transfer rent fees to the protocol treasury
    let rent_amount = ctx.accounts.rent.minimum_balance(TokenAccount::LEN);
    **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? -= rent_amount;
    **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? += rent_amount;

    Ok(())
}
