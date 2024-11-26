use anchor_lang::prelude::*;
use crate::instructions::{get_asset_price::get_asset_price, evaluate_collateral::evaluate_collateral};
use crate::schema::{CollateralAccount, UserAccount, PriceOracle};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub collateral_account: Account<'info, CollateralAccount>,
    pub price_oracle: Account<'info, PriceOracle>,
}

pub fn deposit(ctx: Context<Deposit>, asset: Pubkey, amount: u64) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    let collateral_account = &mut ctx.accounts.collateral_account;

    // Update the user's deposited amount
    user_account.deposited = user_account
        .deposited
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;

    // Fetch the current price of the asset
    let price = get_asset_price(&ctx.accounts.price_oracle, asset)?;

    // Update collateral value based on the deposit
    evaluate_collateral(user_account, collateral_account, price)?;

    emit!(Deposited {
        user: user_account.owner,
        asset,
        amount,
    });

    Ok(())
}



// use anchor_lang::prelude::*;
// use crate::accounts::user_account::UserAccount;


// #[derive(Accounts)]
// pub struct Deposit<'info> {
//     #[account(mut)]
//     pub user_account: Account<'info, UserAccount>,
//     #[account(mut)]
//     pub asset_mint: AccountInfo<'info>,
//     #[account(mut)]
//     pub lending_pool: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
// }

// pub fn deposit(ctx: Context<Deposit>, asset: Pubkey, amount: u64) -> Result<()> {
//     let user_account = &mut ctx.accounts.user_account;

//     // Transfer tokens to the lending pool
//     let ix = spl_token::instruction::transfer(
//         ctx.accounts.system_program.key,
//         ctx.accounts.user_account.to_account_info().key,
//         ctx.accounts.lending_pool.key,
//         &[],
//         amount,
//     )?;
//     anchor_lang::solana_program::program::invoke(&ix, &[
//         ctx.accounts.user_account.to_account_info(),
//         ctx.accounts.lending_pool.clone(),
//     ])?;

//     // Update user balance
//     //user_account.deposited += amount;


//     let user_account = &mut ctx.accounts.user_account;
//     let collateral_account = &mut ctx.accounts.collateral_account;

//     // Update the user's deposited amount
//     user_account.deposited = user_account.deposited.checked_add(amount).ok_or(ErrorCode::Overflow)?;

//     // Fetch the current price of the asset
//     let price = get_asset_price(ctx.accounts.price_oracle, asset)?;
    
//     // Update collateral value based on the deposit
//     evaluate_collateral(ctx, price)?;

//     emit!(Deposited {
//         user: user_account.owner,
//         asset,
//         amount,
//     });

//     Ok(())
// }
