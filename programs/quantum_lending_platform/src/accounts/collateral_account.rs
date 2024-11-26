use anchor_lang::prelude::*;

#[account]
pub struct CollateralAccount {
    pub owner: Pubkey,           // Owner of the collateral
    pub amount: u64,             // Amount of collateral tokens
    pub collateral_value: u64,   // Current value of collateral in USD (or equivalent)
}

impl CollateralAccount {
    pub fn new(owner: Pubkey, amount: u64) -> Self {
        Self {
            owner,
            amount,
            collateral_value: 0, // Value will be set after evaluation
        }
    }
}
