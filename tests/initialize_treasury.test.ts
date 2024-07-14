import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { NftVaultSwap } from "../target/types/nft_vault_swap";
import { assert } from 'chai';
import { expect } from 'chai';

describe('initialize_treasury', () => {
    // Configure the client to user 
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(provider);

    const program = anchor.workspace.NftVaultSwap as Program<NftVaultSwap>
     
    it('Initializes the treasury', async() => {
        const [treasuryPda, _] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from('treasury')],
            program.programId
        )
    
        const rent = new anchor.BN(1000);
        //console.log(treasuryPda)
        await program.methods
        .initializeTreasury(rent)
        .accounts({
            treasury: treasuryPda,
            signer: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId
        })
        .signers([])
        .rpc();
    
        const treasuryAccount = await program.account.treasury.fetch(treasuryPda);
        expect(treasuryAccount.owner.toBase58()).to.equal(provider.wallet.publicKey.toBase58())
        //anchor.BN instances should be compared using toString() to avoid direct comparison issues
        expect(treasuryAccount.rent.toString()).to.equal(rent.toString())
    });   
})
