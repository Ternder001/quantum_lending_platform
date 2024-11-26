use anchor_lang::prelude::*;
use crate::instructions::{get_asset_price::get_asset_price, get_health_factor::get_health_factor};
use crate::schema::{CollateralAccount, UserAccount, PriceOracle};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    pub collateral_account: Account<'info, CollateralAccount>,
    pub price_oracle: Account<'info, PriceOracle>,
}

pub fn borrow(ctx: Context<Borrow>, asset: Pubkey, amount: u64) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    let collateral_account = &ctx.accounts.collateral_account;

    // Fetch the price of the asset
    let price = get_asset_price(&ctx.accounts.price_oracle, asset)?;

    // Calculate maximum borrowable amount based on LTV
    let max_borrow = collateral_account.collateral_value * 75 / 100; // 75% LTV ratio
    require!(amount <= max_borrow, ErrorCode::InsufficientCollateral);

    // Calculate user's health factor after borrowing
    let new_health_factor = get_health_factor(user_account)?;
    require!(new_health_factor >= 50, ErrorCode::LowHealthFactor);

    // Update the borrowed amount
    user_account.borrowed = user_account
        .borrowed
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;

    emit!(Borrowed {
        user: user_account.owner,
        asset,
        amount,
    });

    Ok(())
}



// use anchor_lang::prelude::*;
// use crate::accounts::{user_account::UserAccount, collateral_account::CollateralAccount};

// #[derive(Accounts)]
// pub struct Borrow<'info> {
//     #[account(mut)]
//     pub user_account: Account<'info, UserAccount>,
//     #[account(mut)]
//     pub collateral_account: Account<'info, CollateralAccount>,
//     #[account(mut)]
//     pub lending_pool: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
// }

// pub fn borrow(ctx: Context<Borrow>, asset: Pubkey, amount: u64) -> Result<()> {
//     let user_account = &mut ctx.accounts.user_account;
//     let collateral_account = &ctx.accounts.collateral_account;

//     // Calculate maximum borrowable amount based on collateral
//     let max_borrowable = collateral_account.collateral_value * 75 / 100;
//     require!(user_account.borrowed + amount <= max_borrowable, CustomError::ExceedsBorrowLimit);

//     // Transfer tokens to the user
//     let ix = spl_token::instruction::transfer(
//         ctx.accounts.system_program.key,
//         ctx.accounts.lending_pool.key,
//         ctx.accounts.user_account.to_account_info().key,
//         &[],
//         amount,
//     )?;
//     anchor_lang::solana_program::program::invoke(&ix, &[
//         ctx.accounts.lending_pool.clone(),
//         ctx.accounts.user_account.to_account_info(),
//     ])?;

//     // Update user debt
//     user_account.borrowed += amount;

//     Ok(())
// }
