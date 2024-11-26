use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow error occurred.")]
    Overflow,

    #[msg("Division by zero is not allowed.")]
    DivisionByZero,

    #[msg("Price for the asset is unavailable.")]
    PriceUnavailable,

    #[msg("Interest rate is unavailable.")]
    RateUnavailable,

    #[msg("User has insufficient balance to complete the operation.")]
    InsufficientBalance,

    #[msg("User's collateral is insufficient to support the operation.")]
    InsufficientCollateral,

    #[msg("User's account health factor is too low.")]
    LowHealthFactor,

    #[msg("The repayment amount exceeds the user's outstanding debt.")]
    ExcessRepayment,

    #[msg("Account is healthy and does not require liquidation.")]
    HealthyAccount,

    #[msg("Insufficient Debt.")]
    InsufficientDebt,
}
