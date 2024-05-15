use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct PresaleInfo {
    // Mint address of the presale token
    pub token_mint_address: Pubkey,
    // Mint address of the quote token
    pub quote_token_mint_address: Pubkey,
    // Softcap
    pub softcap_amount: u64,
    // Hardcap
    pub hardcap_amount: u64,

    pub stage_purchased_amount: u64,
    // Total amount of presale tokens available in the presale
    pub deposit_token_amount: u64,
    // Total amount of presale tokens sold during the presale
    pub sold_token_amount: u64,

    pub sold_quote_amount: u64,
    // Start time of presale
    pub start_time: u64,
    // End time of presale
    pub end_time: u64,

    // Minium amount of presale tokens to purchase
    pub min_quote_amount_to_purchase: u64,
    // Quote token per presale token
    pub price_per_token: u64,
    pub stage: u8,
    // Identifier for finding the PDA
    pub identifier: u8,
    // Authority of the presale
    pub authority: Pubkey,
    // Authority of the presale
    pub receiver: Pubkey,
    // Bump used when creating the PDA
    pub bump: u8
}