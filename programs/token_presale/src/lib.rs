use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("3AbJVkSzNn691PS4uLsg8M6WbubbVqYe1oC7KZ7K1ffe");
#[program]
pub mod token_presale {
    use super::*;

    pub fn create_presale(
        ctx: Context<CreatePresale>,
        token_mint_address: Pubkey,
        quote_token_mint_address: Pubkey,
        softcap_amount:u64,
        hardcap_amount:u64,
        min_quote_amount_to_purchase: u64,
        start_time: u64,
        end_time: u64,
        identifier: u8,
        receiver_account: Pubkey
    ) -> Result<()> {
        return create_presale::create_presale(
            ctx,
            token_mint_address,
            quote_token_mint_address,
            softcap_amount,
            hardcap_amount,
            min_quote_amount_to_purchase,
            start_time,
            end_time,
            identifier,
            receiver_account
        );
    }

    pub fn update_presale(
        ctx: Context<UpdatePresale>,
        receiver_account: Pubkey
    ) -> Result<()> {
        return update_presale::update_presale(
            ctx,
            receiver_account
        );
    }

    pub fn deposit_token(
        ctx: Context<DepositToken>,
        amount: u64,
        identifier: u8
    ) -> Result<()> {
        return deposit_token::deposit_token(
            ctx,
            amount,
            identifier
        );
    }

    pub fn buy_token(
        ctx: Context<BuyToken>,
        quote_amount: u64,
        referrer_ddress: Pubkey
    ) -> Result<()> {
        return buy_token::buy_token(
            ctx,
            quote_amount,
            referrer_ddress
        );
    }
}
