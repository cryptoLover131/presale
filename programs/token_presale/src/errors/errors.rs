use anchor_lang::prelude::*;

// Not yet implemented

#[error_code]
pub enum PresaleError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Not allowed")]
    NotAllowed,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Already purchased")]
    AlreadyPurchased,
    #[msg("Presale not started yet")]
    PresaleNotStarted,
    #[msg("Presale already ended")]
    PresaleEnded,
    #[msg("Token amount mismatch")]
    TokenAmountMismatch,
    #[msg("Insufficient Tokens")]
    InsufficientFund,
    #[msg("Presale stage hard cap already reached")]
    StageHardCapReached,
    #[msg("Deposit amount is too small")]
    TooSmallAmount,
    #[msg("Presale not ended yet")]
    PresaleNotEnded,
    #[msg("Presale ID out of range")]
    PresaleIDRange,
    #[msg("Presale stage is incorrect")]
    PresaleStageIncorrect,
    #[msg("Presale hardcap already reached")]
    PresaleHardCapReached,
    #[msg("User hardcap already reached")]
    UserHardCapReached,
    #[msg("SoftCap reached")]
    SoftCapReached,
    #[msg("SoftCap reached")]
    SoftCapNotReached,
    #[msg("Purchase transaction failed")]
    TransactionFailed
}