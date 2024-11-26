use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct FlashLoan<'info> {
    #[account(mut)]
    pub lending_pool: AccountInfo<'info>,        // Lending pool from which funds are borrowed
    #[account(mut)]
    pub borrower: Signer<'info>,                 // The borrower account
    #[account(mut)]
    pub loan_repayment_account: AccountInfo<'info>, // Account to check for loan repayment
    pub system_program: Program<'info, System>,    // System program for token transfers
}

pub fn flash_loan(
    ctx: Context<FlashLoan>,                      // Context containing necessary accounts
    amount: u64,                                  // Amount to be loaned
    execute_logic: Box<dyn FnOnce() -> Result<()>>, // Closure that defines the borrower's custom logic
) -> Result<()> {
    let borrower = &ctx.accounts.borrower;
    let lending_pool = &ctx.accounts.lending_pool;
    let repayment_account = &ctx.accounts.loan_repayment_account;

    // Transfer the loaned amount to the borrower (in SPL tokens)
    let ix = spl_token::instruction::transfer(
        ctx.accounts.system_program.key,
        lending_pool.key,
        borrower.to_account_info().key,
        &[],
        amount,
    )?;
    anchor_lang::solana_program::program::invoke(&ix, &[
        lending_pool.clone(),
        borrower.to_account_info(),
    ])?;

    // Execute the borrower's custom logic
    execute_logic()?;

    // Verify repayment (1% fee assumed)
    let repayment = amount + (amount / 100);
    let balance = repayment_account.lamports();  // Replace this with actual token balance check if needed
    require!(balance >= repayment, CustomError::InsufficientRepayment);

    Ok(())
}
