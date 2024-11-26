use anchor_lang::prelude::*;
use crate::schema::UserAccount;

pub fn update_user_balance(
    user_account: &mut UserAccount,
    collateral_value: u64,
    borrowed_value: u64,
) -> Result<()> {
    user_account.collateral_value = collateral_value;
    user_account.borrowed = borrowed_value;

    Ok(())
}
