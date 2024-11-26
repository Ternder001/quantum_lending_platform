use anchor_lang::prelude::*;
use crate::instructions::{get_health_factor::get_health_factor, get_asset_price::get_asset_price};
use crate::schema::{UserAccount, CollateralAccount, PriceOracle};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    pub collateral_account: Account<'info, CollateralAccount>,
    pub price_oracle: Account<'info, PriceOracle>,
}

pub fn liquidate(ctx: Context<Liquidate>, user: Pubkey) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;

    // Fetch health factor
    let health_factor = get_health_factor(user_account)?;
    require!(health_factor < 50, ErrorCode::HealthyAccount);

    // Fetch the price of the collateral asset
    let price = get_asset_price(&ctx.accounts.price_oracle, ctx.accounts.collateral_account.asset)?;

    // Liquidate the user's collateral to cover outstanding debt
    let liquidation_amount = user_account.borrowed
        .checked_mul(price)
        .ok_or(ErrorCode::Overflow)?
        / 100;

    user_account.borrowed = 0; // Clear user's debt

    emit!(Liquidated {
        user: user_account.owner,
        amount: liquidation_amount,
    });

    Ok(())
}



// use anchor_lang::prelude::*;
// use crate::accounts::{user_account::UserAccount, collateral_account::CollateralAccount};

// #[derive(Accounts)]
// pub struct Liquidate<'info> {
//     #[account(mut)]
//     pub user_account: Account<'info, UserAccount>,
//     #[account(mut)]
//     pub collateral_account: Account<'info, CollateralAccount>,
//     #[account(mut)]
//     pub lending_pool: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
// }

// pub fn liquidate(ctx: Context<Liquidate>, user_pubkey: Pubkey) -> Result<()> {
//     let user_account = &mut ctx.accounts.user_account;

//     // Check if the health factor is below the liquidation threshold
//     require!(user_account.health_factor < 50, CustomError::CannotLiquidateHealthyAccount);

//     // Transfer collateral to the lending pool
//     let collateral_value = ctx.accounts.collateral_account.collateral_value;
//     user_account.collateral = 0;

//     // Emit liquidation event
//     emit!(Liquidated { user: user_pubkey, value: collateral_value });

//     Ok(())
// }

// #[event]
// pub struct Liquidated {
//     pub user: Pubkey,
//     pub value: u64,
// }
