use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use crate::state::treasury::Treasury;
use crate::state::vault::Vault;
use mpl_token_metadata::types::{Collection, Creator, DataV2};
use solana_program::{program::invoke, system_instruction};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account( 
    init,
    payer = payer, 
    mint::decimals = 0,
    mint::authority = vault,
    mint::freeze_authority = vault,
    seeds = [
        "mint".as_bytes(), 
        id.to_le_bytes().as_ref()], 
    bump,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub nft_metadata: UncheckedAccount<'info>,
    #[account(init, payer = payer, seeds = [b"vault", nft_metadata.key().as_ref()], bump, space = 8 + Vault::INIT_SPACE )]
    pub vault: Account<'info, Vault>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = vault,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    pub treasury: Account<'info, Treasury>,
    /// CHECK:
    pub master_edition_account: UncheckedAccount<'info>,

    pub metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<MintNFT>,
    id: u64,
    name: String,
    symbol: String,
    uri: String,
    price: f32, //NFT Price in SOL
                //cant: u64,
) -> Result<()> {
    ctx.accounts.vault.creator = ctx.accounts.payer.key();
    ctx.accounts.vault.price = price;
    ctx.accounts.vault.associated_token_account = ctx.accounts.associated_token_account.key();

    msg!("Creating seeds");
    let id_bytes = id.to_le_bytes();
    let seeds = &["mint".as_bytes(), id_bytes.as_ref(), &[ctx.bumps.mint]];

    msg!("Run mint_to");

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            &[&seeds[..]],
        ),
        1, // 1 token
    )?;

    msg!("Run create metadata accounts v3");

    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                payer: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                mint_authority: ctx.accounts.vault.to_account_info(),
                update_authority: ctx.accounts.vault.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &[&seeds[..]],
        ),
        DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true,
        true,
        None,
    )?;

    msg!("Run create master edition v3");

    create_master_edition_v3(
        CpiContext::new_with_signer(
            ctx.accounts.metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition_account.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                mint_authority: ctx.accounts.vault.to_account_info(),
                update_authority: ctx.accounts.vault.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &[&seeds[..]],
        ),
        Some(1),
    )?;

    msg!("Minted NFT successfully");

    // Paying the rent to the treasury
    invoke(
        &system_instruction::transfer(
            ctx.accounts.payer.to_account_info().key,
            ctx.accounts.treasury.to_account_info().key,
            ctx.accounts.treasury.rent,
        ),
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.treasury.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    Ok(())
}
