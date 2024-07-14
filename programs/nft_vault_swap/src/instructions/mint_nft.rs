use anchor_lang::{prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
// use mpl_token_metadata::types::{Collection, Creator, DataV2};
use mpl_token_metadata::instructions::{CreateCpiBuilder, MintCpiBuilder};
use mpl_token_metadata::{types::{CreateArgs, MintArgs, TokenStandard, PrintSupply}};
// use solana_program::{program::invoke, system_instruction};
use anchor_lang::solana_program::{program::invoke, system_instruction};
use crate::state::*;
use crate::ID;

#[derive(Accounts)]
pub struct MintNFT<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  /// CHECK:
  #[account(mut)]
  pub mint: Signer<'info>,
  /// CHECK:
  #[account(mut)]
  pub metadata_account: AccountInfo<'info>,
  /// CHECK:
  #[account(mut)]
  pub master_edition_account: AccountInfo<'info>,

  pub token_metadata_program: Program<'info, Metadata>,
  pub system_program: Program<'info, System>,
  /// CHECK:
  pub sysvar_program: AccountInfo<'info>,

  /// CHECK:
  #[account(mut)]
  pub associated_token_account: AccountInfo<'info>,

  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,  
  pub rent: Sysvar<'info, Rent>,
  #[
    account
    (
      init, payer = payer, 
      seeds = [b"vault", metadata_account.key().as_ref()], 
      bump, space = 8 + Vault::INIT_SPACE,
      owner = ID
    )
  ]
  pub vault: Account<'info, Vault>,
  #[account(mut)]
  pub treasury: Account<'info, Treasury>,
}

pub fn run_mint_nft(
    ctx: Context<MintNFT>,
    name: String,
    symbol: String,
    uri: String,
    price: f32, //NFT Price in SOL
                //cant: u64,
) -> Result<()> {
  let metadata = &ctx.accounts.metadata_account;

  ctx.accounts.vault.creator = ctx.accounts.payer.key();
  ctx.accounts.vault.price = price;
  ctx.accounts.vault.associated_token_account = ctx.accounts.associated_token_account.key();

  let metadata_binding = metadata.clone().key();
  let seeds: &[&[u8]] = &[b"vault", metadata_binding.as_ref()];
  let (_, bump) = Pubkey::find_program_address(&seeds, &ID);
  let seeds_signer = &mut seeds.to_vec();
  let binding = [bump];
  seeds_signer.push(&binding);

  let creator = &ctx.accounts.payer;
  let mint = &ctx.accounts.mint;
  let token_program = &ctx.accounts.token_program;
  let system_program = &ctx.accounts.system_program;
  
  let authority = &ctx.accounts.vault;
  let create_data = CreateArgs :: V1 {
    name: name.clone(),
    symbol: symbol.clone(),
    uri: uri.clone(),
    seller_fee_basis_points: 0,
    primary_sale_happened: false,
    is_mutable: true,
    token_standard: TokenStandard::NonFungible,
    collection_details: None,
    creators: None,
    collection: None,
    uses: None, 
    decimals: Some(0),
    print_supply: Some(PrintSupply::Unlimited),
    rule_set: None,
  }; 
  
  CreateCpiBuilder::new(&ctx.accounts.token_metadata_program)
  .metadata(metadata)
  .master_edition(Some(&ctx.accounts.master_edition_account))
  .mint(&mint.to_account_info(), true)
  .authority(&authority.to_account_info())
  .payer(&creator.to_account_info())
  .update_authority(&authority.to_account_info(), true)
  .system_program(&system_program.to_account_info())
  .sysvar_instructions(&ctx.accounts.sysvar_program)
  .spl_token_program(Some(&token_program.to_account_info()))
  .create_args(create_data)
  .invoke_signed(&[seeds_signer])?;  

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

  MintCpiBuilder::new(&ctx.accounts.token_metadata_program)
    .token(&ctx.accounts.associated_token_account)
    .token_owner(Some(&ctx.accounts.vault.to_account_info()))
    .metadata(metadata)
    .master_edition(Some(&ctx.accounts.master_edition_account))
    .token_record(None)
    .mint(&ctx.accounts.mint)
    .authority(&ctx.accounts.vault.to_account_info())
    .delegate_record(None)
    .payer(&ctx.accounts.payer)
    .system_program(&ctx.accounts.system_program)
    .sysvar_instructions(&ctx.accounts.sysvar_program)
    .spl_token_program(&ctx.accounts.token_program)
    .spl_ata_program(&ctx.accounts.associated_token_program)
    .authorization_rules(None)
    .authorization_rules_program(None)
    .mint_args(MintArgs::V1 {
      amount: 1,
      authorization_data: None,
    })
    .invoke_signed(&[seeds_signer])?;

  Ok(())
}
