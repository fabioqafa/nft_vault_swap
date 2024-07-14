use anchor_lang::{prelude::*};

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub owner: Pubkey,
    pub rent: u64, //rent is in SOL
}
