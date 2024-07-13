use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;


declare_id!("3haUA3nngbCdg6XH8PbBKg1rXo4a6Q44ak8fabbC7L9j");

#[program]
pub mod nft_vault_swap {
    use super::*;

    pub fn initialize_treasury(ctx: Context<InitializeTreasury>, rent: u64) -> Result<()> {
        instructions::initialize_treasury::handler(ctx, rent)
    }

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        id: u64,
        name: String,
        symbol: String,
        uri: String,
        price: f32,
    ) -> Result<()> {
        instructions::mint_nft::handler(ctx, id, name, symbol, uri, price)
    }

    // pub fn lock_nft(ctx: Context<LockNFT>) -> Result<()> {
    //     instructions::lock_nft::handler(ctx)
    // }

    pub fn swap_sol_for_nft(ctx: Context<SwapSolForNFT>) -> Result<()> {
        instructions::swap_nft::handler(ctx)
    }
}