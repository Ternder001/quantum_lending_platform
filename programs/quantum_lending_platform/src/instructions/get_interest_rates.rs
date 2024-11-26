use anchor_lang::prelude::*;
use crate::schema::LendingPool;

pub fn get_interest_rates(lending_pool: &LendingPool) -> Result<(u64, u64)> {
    // Get the deposit rate (no argument required)
    let deposit_rate = lending_pool.get_deposit_rate();
    
    // Get the borrow rate (no argument required)
    let borrow_rate = lending_pool.get_borrow_rate();
    
    Ok((deposit_rate, borrow_rate))
}

