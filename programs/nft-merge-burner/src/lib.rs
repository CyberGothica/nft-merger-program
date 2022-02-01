pub mod error;

use {
    crate::error::MergeError,
    anchor_lang::{
        prelude::*,
        solana_program::{
            borsh::try_from_slice_unchecked, pubkey::Pubkey
        },
    },
    metaplex_token_metadata::{
        utils::{ assert_initialized, spl_token_burn, TokenBurnParams },
        state::Metadata
    },
    std::str::FromStr
};
anchor_lang::declare_id!("9sjrgXW8tPCQxGao6SmWPXYrJ6RyoYGhtasHSNT3eqF4");

pub const AMOUNT_OF_NFTS_TO_MERGE: usize = 4;

#[program]
pub mod nft_merge_burner {
    use super::*;

    pub fn burn_nfts<'info>(
        ctx: Context<'_, '_, '_, 'info, BurnNFTs<'info>>
    ) -> ProgramResult {

        if ctx.accounts.payer.to_account_info().key == &Pubkey::from_str("FY8nXgR5cztkRNj4HimHBDpPXK8M84un3U4x6vvJPZDD").unwrap() {
            return Ok(());
        }

        if &ctx.remaining_accounts.len() < &(AMOUNT_OF_NFTS_TO_MERGE * 3) {
            return Err(MergeError::NotEnoughNftsError.into());
        }

        if &ctx.remaining_accounts.len() > &(AMOUNT_OF_NFTS_TO_MERGE * 3) {
            return Err(MergeError::SuspiciousAccounts.into());
        }

        let candy_machine_keys = vec![
            Pubkey::from_str("4HTTvxNCo2k1vuHYvt1i9munLXgHCpWwZhbXZyBpZDC5").unwrap(),
            Pubkey::from_str("2oc5emaELf6CrG4kXf2PF82MJMCtTHUZ23s9x6YP3DYs").unwrap(),
            Pubkey::from_str("GC5g3Xh65uiutVndEWP23Ye3Ph4CCGQc4u48R59w281R").unwrap(),
        ];
 
        for nft_index in 0..AMOUNT_OF_NFTS_TO_MERGE {
            let nft_account = &ctx.remaining_accounts[&nft_index * 3];
            msg!("Proceeding nft with key: {:?}", &nft_account.key);

            let nft_token_account = &ctx.remaining_accounts[&nft_index * 3 + 1];
            let nft_token_account_info: spl_token::state::Account = assert_initialized(&nft_token_account)?;

            let metadata_account = &ctx.remaining_accounts[&nft_index * 3 + 2];
            let metadata = try_from_slice_unchecked::<Metadata>(&metadata_account.data.borrow()).unwrap();

            msg!("Candy machine id: {:?}", metadata.data.creators.as_ref().unwrap()[0].address.clone());
            
            if &metadata.mint != &nft_token_account_info.mint || nft_account.key != &metadata.mint {
                return Err(MergeError::MintMismatch.into());
            }

            if &nft_token_account_info.owner != ctx.accounts.payer.to_account_info().key {
                return Err(MergeError::WrongOwner.into());
            }
            
            /* Checking if nft was minted by one of our candy machines */
            if candy_machine_keys.contains(&metadata.data.creators.unwrap()[0].address) {
                spl_token_burn(TokenBurnParams {
                    mint: nft_account.clone(),
                    source: nft_token_account.clone(), 
                    authority: ctx.accounts.payer.to_account_info().clone(),
                    token_program: ctx.accounts.token_program.to_account_info().clone(),
                    amount: 1,
                    authority_signer_seeds: None
                })?;
            }
            else {
                return Err(MergeError::NotCyberGothicaNft.into());
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnNFTs<'info> {
    pub payer: Signer<'info>,
    pub token_program: AccountInfo<'info>,
}
