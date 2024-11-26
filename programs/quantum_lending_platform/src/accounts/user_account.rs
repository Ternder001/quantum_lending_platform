use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub owner: Pubkey,           // Owner of the account
    pub deposited: u64,          // Total deposited assets
    pub borrowed: u64,           // Total borrowed assets
    pub collateral: u64,         // Total collateral provided
    pub health_factor: u64,      // User's health factor (0-100 scale)
}

impl UserAccount {
    pub fn new(owner: Pubkey) -> Self {
        Self {
            owner,
            deposited: 0,
            borrowed: 0,
            collateral: 0,
            health_factor: 100,
        }
    }
}
