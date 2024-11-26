use anchor_lang::prelude::*;

/// User account for storing user-specific data.
#[account]
pub struct UserAccount {
    pub owner: Pubkey,               // The user's public key.
    pub deposited: u64,              // Total amount deposited by the user.
    pub borrowed: u64,               // Total amount borrowed by the user.
    pub collateral_value: u64,       // Current value of the user's collateral.
    pub health_factor: u64,          // User's health factor (scaled up for precision).
    pub last_interest_accrual: i64,  // Timestamp of the last interest accrual.
}

impl UserAccount {
    /// Updates the health factor of the user based on total collateral and debt.
    pub fn update_health_factor(&mut self) {
        if self.borrowed == 0 {
            self.health_factor = u64::MAX;
        } else {
            self.health_factor = self.collateral_value * 1_000_000 / self.borrowed;
        }
    }
}

/// Collateral account for storing collateral-specific data.
#[account]
pub struct CollateralAccount {
    pub owner: Pubkey,               // The user's public key.
    pub asset: Pubkey,               // The asset used as collateral.
    pub collateral_amount: u64,      // Amount of collateral deposited.
    pub collateral_value: u64,       // Current value of the collateral in USD.
}

/// Lending pool account for managing deposits and loans for a specific asset.
#[account]
pub struct LendingPool {
    pub asset: Pubkey,               // The asset managed by the pool.
    pub total_deposits: u64,         // Total deposits in the pool.
    pub total_borrows: u64,          // Total borrows from the pool.
    pub interest_rate: u64,          // Current interest rate (in basis points).
    pub reserve_factor: u64,         // Portion of interest retained as reserves (basis points).
    pub last_interest_accrual: i64,  // Timestamp of the last interest accrual.
}

impl LendingPool {
    /// Calculate the deposit interest rate based on current deposits and reserves
    pub fn get_deposit_rate(&self) -> u64 {
        // Example logic for calculating deposit rate
        let utilization_rate = if self.total_deposits == 0 {
            0
        } else {
            self.total_borrows * 100 / self.total_deposits
        };

        let base_rate = 2; // Base deposit rate in basis points
        let variable_rate = utilization_rate / 2; // Example variable rate logic

        base_rate + variable_rate
    }

    /// Calculate the borrow interest rate based on current borrows and reserves
    pub fn get_borrow_rate(&self) -> u64 {
        // Example logic for calculating borrow rate
        let utilization_rate = if self.total_deposits == 0 {
            0
        } else {
            self.total_borrows * 100 / self.total_deposits
        };

        let base_rate = 5; // Base borrow rate in basis points
        let variable_rate = utilization_rate; // Example variable rate logic

        base_rate + variable_rate
    }
}


/// Price Oracle account for fetching and storing asset prices.
#[account]
pub struct PriceOracle {
    pub prices: Vec<(Pubkey, u64)>, // List of asset-price pairs.
}

impl PriceOracle {
    /// Fetches the price of a specific asset.
    pub fn get_price(&self, asset: &Pubkey) -> Option<u64> {
        self.prices.iter().find_map(|(key, price)| if key == asset { Some(*price) } else { None })
    }

    /// Updates the price of a specific asset.
    pub fn update_price(&mut self, asset: Pubkey, price: u64) {
        if let Some((_, stored_price)) = self.prices.iter_mut().find(|(key, _)| *key == asset) {
            *stored_price = price;
        } else {
            self.prices.push((asset, price));
        }
    }
}

