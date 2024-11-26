use anchor_lang::prelude::*;
use crate::schema::UserAccount;
use crate::error::ErrorCode;

pub fn get_health_factor(user_account: &UserAccount) -> Result<u64> {
    if user_account.borrowed == 0 {
        return Ok(u64::MAX);
    }

    let health_factor = user_account
        .collateral_value
        .checked_mul(100)
        .ok_or(ErrorCode::Overflow)?
        .checked_div(user_account.borrowed)
        .ok_or(ErrorCode::DivisionByZero)?;

    Ok(health_factor)
}
