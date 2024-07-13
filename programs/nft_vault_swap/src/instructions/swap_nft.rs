use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::vault::Vault;
use solana_program::{program::invoke, system_instruction};

#[derive(Accounts)]
pub struct SwapSolForNFT<'info> {
    #[account(mut, signer)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<SwapSolForNFT>) -> Result<()> {
    // Transfer SOL from user to vault
    const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;
    let price = (ctx.accounts.vault.price as f64 * LAMPORTS_PER_SOL).round() as u64;

    invoke(
        &system_instruction::transfer(
            ctx.accounts.buyer.to_account_info().key,
            ctx.accounts.vault.to_account_info().key,
            price,
        ),
        &[
            ctx.accounts.buyer.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Transfer NFT from vault to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, 1)?;

    Ok(())
}
