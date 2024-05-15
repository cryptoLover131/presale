use {
    anchor_lang::prelude::*,
    anchor_spl::{
        token,
        associated_token,
    },
};

use crate::state::PresaleInfo;
use crate::constants::{PRESALE_SEED, PRESALE_RESERVE_SEED, ESCROW_SEED};

pub fn deposit_token(
    ctx: Context<DepositToken>, 
    amount: u64,
    identifier: u8,
) -> Result<()> {

    msg!("Depositing presale tokens to presale {}...", identifier);
    msg!("Mint: {}", &ctx.accounts.mint_account.to_account_info().key());   
    msg!("From Token Address: {}", &ctx.accounts.from_associated_token_account.key());     
    msg!("To Token Address: {}", &ctx.accounts.to_associated_token_account.key());     
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.from_associated_token_account.to_account_info(),
                to: ctx.accounts.to_associated_token_account.to_account_info(),
                authority: ctx.accounts.from_authority.to_account_info(),
            },
        ),
        amount,
    )?;

    let presale_info = &mut ctx.accounts.presale_info;

    presale_info.deposit_token_amount = presale_info.deposit_token_amount + amount;

    msg!("Tokens deposited successfully.");

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    amount: u64,
    identifier: u8,
)]
pub struct DepositToken<'info> {
    #[account(mut)]
    pub mint_account: Account<'info, token::Mint>,
    
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = from_authority,
    )]
    pub from_associated_token_account: Box<Account<'info, token::TokenAccount>>,

    #[account(constraint = payer.key() == from_authority.key())]
    pub from_authority: SystemAccount<'info>,

    #[account(
        // mut,
        init_if_needed,
        payer = payer,
        token::mint = mint_account,
        token::authority = presale_info,
        seeds = [ESCROW_SEED, presale_info.key().as_ref()],
        bump
    )]
    pub to_associated_token_account: Box<Account<'info, token::TokenAccount>>,

    #[account(
        mut,
        seeds = [PRESALE_SEED, from_authority.key().as_ref()],
        bump = presale_info.bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    #[account(
        mut,
        seeds = [PRESALE_RESERVE_SEED, from_authority.key().as_ref()],
        bump = presale_info.identifier
    )]
    pub presale_reserve_pda: SystemAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}