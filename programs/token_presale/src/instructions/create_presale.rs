use anchor_lang::prelude::*;

use crate::state::PresaleInfo;
use crate::constants::{ PRESALE_SEED, PRESALE_RESERVE_SEED, PRESALE_ID_MAX };
use crate::errors::PresaleError;
use std::str::FromStr;

// Edit the details for a presale
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
    
    let presale_info = &mut ctx.accounts.presale_info;
    let authority = &ctx.accounts.authority;

    // Check valid Presale ID
    if identifier < 1 || identifier > PRESALE_ID_MAX {
        msg!("Presale ID out of range.");
        return Err(PresaleError::PresaleIDRange.into());
    }

    // Set the presale details to the parameters given
    // presale_info.is_live = false;
    presale_info.token_mint_address = token_mint_address;
    presale_info.quote_token_mint_address = quote_token_mint_address;
    presale_info.deposit_token_amount = 0;
    presale_info.sold_token_amount = 0;
    presale_info.start_time = start_time;
    presale_info.end_time = end_time;
    presale_info.softcap_amount = softcap_amount;
    presale_info.hardcap_amount = hardcap_amount;
    presale_info.stage_purchased_amount = 0;
    presale_info.min_quote_amount_to_purchase = min_quote_amount_to_purchase;
    presale_info.price_per_token = 7500;
    presale_info.authority = authority.key();
    presale_info.stage = 1;
    presale_info.identifier = *ctx.bumps.get("presale_reserve_pda").unwrap();
    presale_info.bump = *ctx.bumps.get("presale_info").unwrap();
    presale_info.receiver = receiver_account;

    msg!(
        "Presale has created for token: {}",
        presale_info.token_mint_address
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    token_mint_address: Pubkey,
    quote_token_mint_address: Pubkey,
    softcap_amount:u64,
    hardcap_amount:u64,
    min_quote_amount_to_purchase: u64,
    start_time: u64,
    end_time: u64,
    identifier: u8,
    receiver_account: Pubkey
)]
pub struct CreatePresale<'info> {
    // Initialize the presale_info account
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + std::mem::size_of::<PresaleInfo>(),
        seeds = [PRESALE_SEED.as_ref(), authority.key().as_ref()],
        bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    #[account(
        seeds = [PRESALE_RESERVE_SEED, authority.key().as_ref()],
        bump
    )]
    pub presale_reserve_pda: SystemAccount<'info>,

    // Set the authority to the transaction signer
    #[account
    (
        mut
    )]
    pub authority: Signer<'info>,
    
    // Must be included when initializing an account
    pub system_program: Program<'info, System>,
}