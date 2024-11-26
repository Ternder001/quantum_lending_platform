use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::schema::{UserAccount, LendingPool};

pub fn accrue_interest(
    user_account: &mut UserAccount,
    lending_pool: &mut LendingPool,
    current_rate: u64,
) -> Result<()> {
    // Get the current timestamp from the Solana clock
    let current_time = Clock::get()?.unix_timestamp;

    // Calculate the time elapsed since the last interest accrual
    let time_elapsed = current_time - lending_pool.last_interest_accrual;

    if time_elapsed > 0 {
        // Calculate the interest based on elapsed time and the current rate
        let interest = user_account.borrowed
            .checked_mul(current_rate)
            .ok_or(ErrorCode::Overflow)?
            .checked_mul(time_elapsed as u64)
            .ok_or(ErrorCode::Overflow)?
            / 1_000_000_000; // Scale appropriately for fractional values

        // Update the user's borrowed amount with the accrued interest
        user_account.borrowed = user_account
            .borrowed
            .checked_add(interest)
            .ok_or(ErrorCode::Overflow)?;

        // Update the lending pool's last interest accrual timestamp
        lending_pool.last_interest_accrual = current_time;
    }

    Ok(())
}
