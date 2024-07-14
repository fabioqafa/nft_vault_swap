pub mod state;
pub mod instructions;

use anchor_lang::{prelude::*};
use instructions::*;

declare_id!("3haUA3nngbCdg6XH8PbBKg1rXo4a6Q44ak8fabbC7L9j");

#[program]
pub mod nft_vault_swap {
    use super::*;

    pub fn initialize_treasury(ctx: Context<InitializeTreasury>, rent: u64) -> Result<()> {
        run_initialize_treasury(ctx, rent)
    }

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        name: String,
        symbol: String,
        uri: String,
        price: f32,
    ) -> Result<()> {
        run_mint_nft(ctx, name, symbol, uri, price)
    }

    pub fn swap_sol_for_nft(ctx: Context<SwapSolForNFT>) -> Result<()> {
        run_swap_nft(ctx)
    }
}