use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{
        token,
        associated_token,
    },
};
use solana_program::clock::Clock;

use crate::state::PresaleInfo;
use crate::state::UserInfo;
use crate::constants::{PRESALE_SEED, USER_SEED};
use crate::errors::PresaleError;

pub fn buy_token(
    ctx: Context<BuyToken>,
    quote_amount: u64,
    referrer_ddress: Pubkey
) -> Result<()> {
    let presale_info = &mut ctx.accounts.presale_info;
    // const solReciever = Pubkey::from_str("8tFunKMZagDsCRgKusmtdNPcPW2ReEzr7RvuV5hK6kbD").unwrap();
    let bump = &[presale_info.bump];
    let user_info = &mut ctx.accounts.user_info;
    let cur_timestamp = u64::try_from(Clock::get()?.unix_timestamp).unwrap();
    let receiver_account: &mut SystemAccount<> = &mut ctx.accounts.sol_receiver;
    let quote_token_amount: u64;

    if presale_info.sold_token_amount <= 50000000000000000 {
        if  (presale_info.sold_token_amount + quote_amount * presale_info.price_per_token) <= 50000000000000000 {
            quote_token_amount = quote_amount * presale_info.price_per_token;
        }
        else {
            presale_info.stage = 2;
            let quote_token_amount1 = 50000000000000000 - presale_info.sold_token_amount;
            let quote_token_amount2 = (quote_amount - quote_token_amount1 / presale_info.price_per_token) * 5000;
            quote_token_amount = quote_token_amount1 + quote_token_amount2;
            presale_info.price_per_token = 5000;
        }
    } else if presale_info.sold_token_amount <= 100000000000000000 {
        presale_info.stage = 2;
        presale_info.price_per_token = 5000;
        if  (presale_info.sold_token_amount + quote_amount * presale_info.price_per_token) <= 100000000000000000 {
            quote_token_amount = quote_amount * presale_info.price_per_token;
        }
        else {
            presale_info.stage = 3;
            let quote_token_amount1 = 100000000000000000 - presale_info.sold_token_amount;
            let quote_token_amount2 = (quote_amount - quote_token_amount1 / presale_info.price_per_token) * 3750;
            quote_token_amount = quote_token_amount1 + quote_token_amount2;
            presale_info.price_per_token = 3750;
        }
    } else if presale_info.sold_token_amount <= 200000000000000000 {
        presale_info.stage = 3;  
        presale_info.price_per_token = 3750;
        if  (presale_info.sold_token_amount + quote_amount * presale_info.price_per_token) <= 200000000000000000 {
            quote_token_amount = quote_amount * presale_info.price_per_token;
        }
        else {
            presale_info.stage = 4;
            let quote_token_amount1 = 200000000000000000 - presale_info.sold_token_amount;
            let quote_token_amount2 = (quote_amount - quote_token_amount1 / presale_info.price_per_token) * 2500;
            quote_token_amount = quote_token_amount1 + quote_token_amount2;
            presale_info.price_per_token = 2500;
        }          
    } else if presale_info.sold_token_amount <= 300000000000000000 {
        presale_info.stage = 4;    
        presale_info.price_per_token = 2500;
        if  (presale_info.sold_token_amount + quote_amount * presale_info.price_per_token) <= 300000000000000000 {
            quote_token_amount = quote_amount * presale_info.price_per_token;
        }
        else {
            presale_info.stage = 5;
            let quote_token_amount1 = 300000000000000000 - presale_info.sold_token_amount;
            let quote_token_amount2 = (quote_amount - quote_token_amount1 / presale_info.price_per_token) * 1875;
            quote_token_amount = quote_token_amount1 + quote_token_amount2;
            presale_info.price_per_token = 1875;
        }           
    } else if presale_info.sold_token_amount <= 400000000000000000 {
        presale_info.stage = 5;
        presale_info.price_per_token = 1875;
        if  (presale_info.sold_token_amount + quote_amount * presale_info.price_per_token) <= 400000000000000000 {
            quote_token_amount = quote_amount * presale_info.price_per_token;
        }
        else {
            presale_info.stage = 6;
            let quote_token_amount1 = 400000000000000000 - presale_info.sold_token_amount;
            let quote_token_amount2 = (quote_amount - quote_token_amount1 / presale_info.price_per_token) * 1500;
            quote_token_amount = quote_token_amount1 + quote_token_amount2;
            presale_info.price_per_token = 1500;
        }              
    } else {
        presale_info.stage = 6;  
        presale_info.price_per_token = 1500;
        quote_token_amount = quote_amount * presale_info.price_per_token;          
    }

    // get time and compare with start and end time
    if presale_info.start_time > cur_timestamp {
        msg!("Presale not started yet.");
        return Err(PresaleError::PresaleNotStarted.into());
    }

    if presale_info.end_time < cur_timestamp {
        msg!("Presale already ended.");
        return Err(PresaleError::PresaleEnded.into())
    }

    if presale_info.min_quote_amount_to_purchase > quote_amount {
        msg!("Min buy amount is 0.1 SOL");
        return Err(PresaleError::InsufficientFund.into())
    }

    if receiver_account.key() != presale_info.receiver.key(){
        msg!("Purchase transaction failed!");
        return Err(PresaleError::TransactionFailed.into())
    }

    user_info.referral_address = referrer_ddress;

    // send quote token(SOL) to contract and update the user info
    user_info.buy_time = cur_timestamp;
    user_info.buy_quote_amount = user_info.buy_quote_amount + quote_amount;
    user_info.buy_token_amount = user_info.buy_token_amount + quote_token_amount;

    presale_info.sold_token_amount = presale_info.sold_token_amount + quote_token_amount;
    presale_info.sold_quote_amount = presale_info.sold_quote_amount + quote_amount;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.sol_receiver.to_account_info(),
            })
        , quote_amount
    )?;

    msg!("Presale tokens transferred successfully.");

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.presale_presale_token_associated_token_account.to_account_info(),
                to: ctx.accounts.buyer_presale_token_associated_token_account.to_account_info(),
                authority: ctx.accounts.presale_info.to_account_info(),
            },
            &[&[PRESALE_SEED, ctx.accounts.presale_authority.key().as_ref(), bump][..]],
        ),
        quote_token_amount,
    )?;

    msg!("Presale tokens transferred successfully.");

    Ok(())
}


#[derive(Accounts)]
#[instruction(
    quote_amount: u64,
    referrer_ddress: Pubkey
)]
pub struct BuyToken<'info> {
    #[account(
        mut,
        seeds = [PRESALE_SEED, presale_authority.key().as_ref()],
        bump = presale_info.bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    #[account(mut)]
    pub presale_token_mint_account: Box<Account<'info, token::Mint>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = presale_token_mint_account,
        associated_token::authority = buyer_authority,
    )]
    pub buyer_presale_token_associated_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub presale_presale_token_associated_token_account: Box<Account<'info, token::TokenAccount>>,
    
    pub presale_authority: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + std::mem::size_of::<UserInfo>(),
        seeds = [
            USER_SEED, 
            presale_authority.key().as_ref(),
            buyer.key().as_ref()
        ],
        bump
    )]
    pub user_info: Box<Account<'info, UserInfo>>,
    #[account(constraint = buyer.key() == buyer_authority.key())]
    pub buyer_authority: SystemAccount<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        mut,
    )]
    pub sol_receiver: SystemAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}