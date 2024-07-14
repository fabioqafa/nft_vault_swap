use anchor_lang::{prelude::*};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;
// use solana_program::{program::invoke, system_instruction};
use anchor_lang::solana_program::{program::invoke, system_instruction};
use crate::state::*;
use crate::ID;

#[derive(Accounts)]
pub struct SwapSolForNFT<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    /// CHECK:
    #[account(mut)]
    pub buyer_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub ata_program: Program<'info, AssociatedToken>,
    ///CHECK:
    pub metadata_account: AccountInfo<'info>,
    /// CHECK:
    pub mint_account: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn run_swap_nft(ctx: Context<SwapSolForNFT>) -> Result<()> {
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
    let metadata = &ctx.accounts.metadata_account;
    let metadata_binding = metadata.clone().key();

    let seeds: &[&[u8]] = &[b"vault", metadata_binding.as_ref()];
    let (_, bump) = Pubkey::find_program_address(&seeds, &ID);
    
    let signer_seeds: &[&[&[u8]]] = &[&[b"vault", metadata_binding.as_ref(), &[bump]]];

    // Transfer NFT from vault to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts)
    .with_signer(signer_seeds);
    token::transfer(cpi_ctx, 1)?;
    
    Ok(())
}
