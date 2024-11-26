use anchor_lang::prelude::*;
use crate::error::ErrorCode;


pub fn withdraw(ctx: Context<Withdraw>, asset: Pubkey, amount: u64) -> Result<()> {
    let lending_pool = &mut ctx.accounts.lending_pool;
    let user_account = &mut ctx.accounts.user_account;

    // Ensure the user has enough balance to withdraw
    if user_account.deposited < amount {
        return Err(ErrorCode::InsufficientBalance.into());
    }

    // Update the user's deposited balance
    user_account.deposited = user_account.deposited.checked_sub(amount).ok_or(ErrorCode::Overflow)?;

    // Update the lending pool's total deposits
    lending_pool.total_deposits = lending_pool
        .total_deposits
        .checked_sub(amount)
        .ok_or(ErrorCode::Overflow)?;

    // Emit a Withdrawn event
    emit!(Withdrawn {
        user: user_account.owner,
        amount,
    });

    Ok(())
}
