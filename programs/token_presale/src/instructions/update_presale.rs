use anchor_lang::prelude::*;

use crate::state::PresaleInfo;
use crate::constants::{PRESALE_SEED, PRESALE_RESERVE_SEED};
use std::str::FromStr;

// Edit the details for a presale
pub fn update_presale(
    ctx: Context<UpdatePresale>,
    receiver_account: Pubkey
) -> Result<()> {
    
    let presale_info = &mut ctx.accounts.presale_info;
    presale_info.receiver = receiver_account;

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    receiver_account: Pubkey,
)]
pub struct UpdatePresale<'info> {
    // Initialize the presale_detils account
    #[account(
        // init,
        // payer = authority,
        // space = 8 + std::mem::size_of::<PresaleInfo>(),
        // seeds = [PRESALE_SEED, authority.key().as_ref()],
        // bump
        mut,
        seeds = [PRESALE_SEED, authority.key().as_ref()],
        bump = presale_info.bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    #[account(
        seeds = [PRESALE_RESERVE_SEED, authority.key().as_ref()],
        bump = presale_info.identifier
    )]
    pub presale_reserve_pda: SystemAccount<'info>,
    
    // Set the authority to the transaction signer
    #[account(
        mut
    )]
    pub authority: Signer<'info>,
    
    // Must be included when initializing an account
    pub system_program: Program<'info, System>,
}