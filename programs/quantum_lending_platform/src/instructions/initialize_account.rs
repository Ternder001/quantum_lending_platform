use anchor_lang::prelude::*;
use crate::schema::UserAccount;

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_account(ctx: Context<InitializeAccount>, user_pubkey: Pubkey) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    user_account.owner = user_pubkey;
    user_account.deposited = 0;
    user_account.borrowed = 0;
    user_account.collateral_value = 0;
    user_account.health_factor = 100; // Default health factor
    Ok(())
}
