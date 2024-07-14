use anchor_lang::{prelude::*};

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub creator: Pubkey,
    pub price: f32, //NFT price in SOL
    pub associated_token_account: Pubkey,
}
