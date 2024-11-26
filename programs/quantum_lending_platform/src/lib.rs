use anchor_lang::prelude::*;
//use instructions::*;

pub mod instructions;
pub mod error;
pub mod schema;

#[program]
pub mod quantum_lending_platform {
    use super::*;

    pub fn initialize_account(ctx: Context<InitializeAccount>, user_pubkey: Pubkey) -> Result<()> {
        instructions::initialize_account::initialize_account(ctx, user_pubkey)
    }

    pub fn deposit(ctx: Context<Deposit>, asset: Pubkey, amount: u64) -> Result<()> {
        instructions::deposit::deposit(ctx, asset, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, asset: Pubkey, amount: u64) -> Result<()> {
        instructions::withdraw::withdraw(ctx, asset, amount)
    }

    pub fn borrow(ctx: Context<Borrow>, asset: Pubkey, amount: u64) -> Result<()> {
        instructions::borrow::borrow(ctx, asset, amount)
    }

    pub fn repay(ctx: Context<Repay>, asset: Pubkey, amount: u64) -> Result<()> {
        instructions::repay::repay(ctx, asset, amount)
    }

    pub fn liquidate(ctx: Context<Liquidate>, user_pubkey: Pubkey) -> Result<()> {
        instructions::liquidate::liquidate(ctx, user_pubkey)
    }

    pub fn accrue_interest(
        ctx: Context<AccrueInterest>,
        current_rate: u64,
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let lending_pool = &mut ctx.accounts.lending_pool;
    
        instructions::accrue_interest::accrue_interest(
            user_account,
            lending_pool,
            current_rate,
        )
    }
    

    pub fn get_asset_price(ctx: Context<GetAssetPrice>, asset: Pubkey) -> Result<u64> {
        instructions::get_asset_price::get_asset_price(ctx, asset)
    }    

    pub fn get_health_factor(ctx: Context<GetHealthFactor>) -> Result<u64> {
        let user_account = &ctx.accounts.user_account;
    
        instructions::get_health_factor::get_health_factor(user_account)
    }
    

    pub fn get_interest_rate(ctx: Context<GetInterestRate>) -> Result<(u64, u64)> {
        let lending_pool = &ctx.accounts.lending_pool;
    
        let (deposit_rate, borrow_rate) = instructions::get_interest_rate::get_interest_rate(lending_pool)?;
        msg!("Deposit Rate: {}, Borrow Rate: {}", deposit_rate, borrow_rate);
    
        Ok((deposit_rate, borrow_rate))
    }
    
    pub fn flash_loan(ctx: Context<FlashLoan>, amount: u64, custom_logic_data: Vec<u8>,) -> Result<()> {
        // Call the flash_loan function in the instructions module
        flash_loan(ctx, amount, custom_logic_data)
    }

}