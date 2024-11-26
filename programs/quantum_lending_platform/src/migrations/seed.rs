use anchor_lang::prelude::*;

pub fn seed(ctx: Context<Seed>) -> ProgramResult {
    let lending_pool = &mut ctx.accounts.lending_pool;
    lending_pool.authority = ctx.accounts.authority.key();
    lending_pool.collateral_mint = ctx.accounts.collateral_mint.key();
    lending_pool.liquidity_mint = ctx.accounts.liquidity_mint.key();
    lending_pool.interest_rate = 0.05;
    lending_pool.liquidation_threshold = 0.5;
    lending_pool.liquidation_penalty = 0.1;
    Ok(())
}

#[derive(Accounts)]
pub struct Seed<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 8 + 8 + 8)]
    pub lending_pool: Account<'info, LendingPool>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub collateral_mint: Account<'info, Mint>,
    pub liquidity_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}