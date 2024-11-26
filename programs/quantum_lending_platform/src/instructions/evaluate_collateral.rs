use anchor_lang::prelude::*;
use crate::schema::{CollateralAccount, UserAccount};
use crate::error::ErrorCode;

pub fn evaluate_collateral(
    user_account: &mut UserAccount,
    collateral_account: &mut CollateralAccount,
    price: u64,
) -> Result<()> {
    collateral_account.collateral_value = collateral_account
        .collateral_amount
        .checked_mul(price)
        .ok_or(ErrorCode::Overflow)?;

    user_account.collateral_value = collateral_account.collateral_value;
    Ok(())
}
