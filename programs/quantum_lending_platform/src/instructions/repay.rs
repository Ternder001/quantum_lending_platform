use anchor_lang::prelude::*;
use crate::error::ErrorCode;

pub fn repay( ctx: Context<Repay>, asset: Pubkey, amount: u64 ) -> Result<()> {
    let lending_pool = &mut ctx.accounts.lending_pool;
    let user_account = &mut ctx.accounts.user_account;

    // if user_account.deposited < amount {
    //     return Err(ErrorCode::InsufficientBalance.into());
    // }

    // Step 2: Ensure that the user has enough debt to repay
    require!(
        user_account.borrowed >= amount,
        ErrorCode::InsufficientDebt
    );

    // Step 3: Deduct the amount from the user account's debt
    user_account.borrowed = user_account
        .borrowed
        .checked_sub(amount)
        .ok_or(ErrorCode::Overflow)?;

    // Step 4: Add the repaid amount to the lending pool's total borrows
    lending_pool.total_borrows = lending_pool
        .total_borrows
        .checked_sub(amount)
        .ok_or(ErrorCode::Overflow)?;

     // Update user balance
    update_user_balance(user_account, user_account.collateral_value, user_account.borrowed)?;

    emit!(Repaid {
        user: user_account.owner,
        asset,
        amount,
    });

    msg!(
        "Repayment of {} successfully processed. Updated debt: {}.",
        amount,
        user_account.borrowed
    );

    Ok(())
}
