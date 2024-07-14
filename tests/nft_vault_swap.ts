import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { NftVaultSwap } from '../target/types/nft_vault_swap';
import { assert } from 'chai';
import { expect } from 'chai';
import { min } from 'bn.js';
import * as splToken from '@solana/spl-token';

describe('testing nft_vault_swap', () => {
  // Configure the client to user
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const payer = anchor.workspace.NftVaultSwap.provider.wallet
    .payer as anchor.web3.Keypair;

  const program = anchor.workspace.NftVaultSwap as Program<NftVaultSwap>;
  const [treasuryPda, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('treasury')],
    program.programId
  );
  const MetadataProgram = new anchor.web3.PublicKey(
    'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s' // This is the known address for the Metaplex Metadata program
  );
  const AssociatedTokenProgram = new anchor.web3.PublicKey(
    'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL' // This is the known address for the Metaplex Associated Token program
  );
  const TokenProgram = new anchor.web3.PublicKey(
    'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA' // This is the known address for the SPL Token program
  );
  const mintKp = anchor.web3.Keypair.generate();
  const [metadataAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      MetadataProgram.toBuffer(),
      mintKp.publicKey.toBuffer(),
    ],
    MetadataProgram
  );
  before(async () => {
    // fetch MetadataProgram account
    const mpl = await provider.connection.getAccountInfo(MetadataProgram);
    const atp = await provider.connection.getAccountInfo(
      AssociatedTokenProgram
    );
  });
  it('Initializes the treasury', async () => {
    const rent = new anchor.BN(1000);
    const accounts = {
      treasury: treasuryPda,
      signer: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    };
    await program.methods
      .initializeTreasury(rent)
      .accounts(accounts)
      .signers([])
      .rpc();

    const treasuryAccount = await program.account.treasury.fetch(treasuryPda);
    expect(treasuryAccount.owner.toBase58()).to.equal(
      provider.wallet.publicKey.toBase58()
    );
    //anchor.BN instances should be compared using toString() to avoid direct comparison issues
    expect(treasuryAccount.rent.toString()).to.equal(rent.toString());
  });
  it('Mint a NFT', async () => {
    const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('vault'), metadataAccount.toBuffer()],
      program.programId
    );
    // change the owner from SystemProgram to SPLTokenProgram & create data for the mint
    // HACK: is this really a correct way to create a mint?
    const mint = await splToken.createMint(
      provider.connection,
      payer,
      vault,
      vault,
      0,
      mintKp
    );
    const tokenAccount = anchor.web3.Keypair.generate().publicKey;
    const [masterEditionAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from('metadata'),
        MetadataProgram.toBuffer(),
        mintKp.publicKey.toBuffer(),
        Buffer.from('edition'),
      ],
      MetadataProgram
    );
    const [associatedTokenAccount] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          vault.toBuffer(),
          TokenProgram.toBuffer(),
          mintKp.publicKey.toBuffer(),
        ],
        AssociatedTokenProgram
      );
    console.log('Mint Account: ', mintKp.publicKey.toBase58());
    console.log('Metadata Account: ', metadataAccount.toBase58());
    console.log('Vault Account: ', vault.toBase58());
    console.log('Token Account: ', tokenAccount.toBase58());
    console.log('Master Edition Account: ', masterEditionAccount.toBase58());
    console.log('Treasury Account: ', treasuryPda.toBase58());
    console.log(
      'Associated Token Account: ',
      associatedTokenAccount.toBase58()
    );
    const accounts = {
      payer: provider.wallet.publicKey,
      mint: mintKp.publicKey,
      metadataAccount,
      vault,
      tokenAccount,
      treasury: treasuryPda,
      masterEditionAccount,
      associatedTokenAccount,
      metadataProgram: MetadataProgram,
      associatedTokenProgram: AssociatedTokenProgram,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      sysvarProgram: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
    };
    await program.methods
      .mintNft('superteam UK', 'STUK', 'uri_test', 1.5)
      .accounts(accounts)
      .signers([payer, mintKp])
      .rpc();
  });
  it('Swap NFT', async () => {
    const [metadataAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from('metadata'),
        MetadataProgram.toBuffer(),
        mintKp.publicKey.toBuffer(),
      ],
      MetadataProgram
    );
    const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('vault'), metadataAccount.toBuffer()],
      program.programId
    );
    const buyer = anchor.web3.Keypair.generate();
    const [buyerTokenAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        buyer.publicKey.toBuffer(),
        TokenProgram.toBuffer(),
        mintKp.publicKey.toBuffer(),
      ],
      AssociatedTokenProgram
    );

    const [vaultTokenAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [vault.toBuffer(), TokenProgram.toBuffer(), mintKp.publicKey.toBuffer()],
      AssociatedTokenProgram
    );

    const accounts = {
      buyer: buyer.publicKey,
      vault,
      buyerTokenAccount,
      vaultTokenAccount,
      mintAccount: mintKp.publicKey,
      ataProgram: AssociatedTokenProgram,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      metadataAccount,
    };

    const systemProgram = anchor.web3.SystemProgram;
    let txn = new anchor.web3.Transaction();
    txn.add(
      systemProgram.transfer({
        fromPubkey: payer.publicKey,
        toPubkey: buyer.publicKey,
        lamports: 10_000_000_000, // 10 SOL
      })
    );
    await anchor.web3.sendAndConfirmTransaction(
      program.provider.connection,
      txn,
      [payer]
    );
    await splToken.createAssociatedTokenAccount(
      program.provider.connection,
      buyer,
      mintKp.publicKey,
      buyer.publicKey
    );
    console.log('Buyer: ', buyer.publicKey.toBase58());
    console.log('Token Account of the Buyer: ', buyerTokenAccount.toBase58());
    console.log('Mint Account: ', mintKp.publicKey.toBase58());
    await program.methods
      .swapSolForNft()
      .accounts(accounts)
      .signers([buyer])
      .rpc();
    // change the owner from SystemProgram to SPLTokenProgram & create data for the mint
  });
});
