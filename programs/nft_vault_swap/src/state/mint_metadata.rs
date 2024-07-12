use anchor_lang::prelude::*;

#[account]
pub struct MintMetadata {
    pub metadata: String,
}
