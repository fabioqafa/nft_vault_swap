use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub price: u64,
}
