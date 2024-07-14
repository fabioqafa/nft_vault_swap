# NFT Vault Swap

## Setup

Step 1. Run `prepare.sh` to dump `metaplex_token_metadata_program` to localcluster.
Step 2. Change the wallet in Anchor.toml to your wallet
Step 3. Run `anchor build`
Step 4. Run `yarn` to install cli
Step 5. Run `anchor test`

## Program structures

### Instructions

- Initialize treasury: To create a treasury for the owner of the program. Keep the value of the `rent: u64`. Everytime creator mint a NFT, he has to pay a rent, storing in this account.
- Mint NFT: There are 2 steps inside this instruction: creating the NFT and mint the NFT, hence storing in a `vault` owned by this program.
- Swap NFT: Buyer can pay the `price` defined by the creator and take the NFT for himself.

### State

- Treasury: owner, rent
- Vault: creator, price, associated_token_account (this is for the future UI, not for the command line application)
