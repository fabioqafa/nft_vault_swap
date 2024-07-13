use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;


declare_id!("3haUA3nngbCdg6XH8PbBKg1rXo4a6Q44ak8fabbC7L9j");

#[program]
pub mod nft_vault_swap {
    use super::*;
    
    // pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
    //     instructions::initialize_vault::handler(ctx)
    // }

    pub fn mint_nft(ctx: Context<MintNFT>, metadata: String) -> Result<()> {
        instructions::mint_nft::handler(ctx, metadata)
    }

    pub fn lock_nft(ctx: Context<LockNFT>) -> Result<()> {
        instructions::lock_nft::handler(ctx)
    }

    pub fn swap_sol_for_nft(ctx: Context<SwapSolForNFT>, amount: u64) -> Result<()> {
        instructions::swap_nft::handler(ctx, amount)
    }
}