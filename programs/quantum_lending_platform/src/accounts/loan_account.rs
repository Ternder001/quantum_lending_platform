use anchor_lang::prelude::*;

#[account]
pub struct LoanAccount {
    pub borrower: Pubkey,        // Address of the borrower
    pub loan_amount: u64,        // Amount of tokens borrowed
    pub interest_rate: u64,      // Interest rate applied to the loan (in basis points)
    pub due_date: i64,           // UNIX timestamp for loan repayment deadline
    pub is_active: bool,         // Whether the loan is active
}

impl LoanAccount {
    pub fn new(borrower: Pubkey, loan_amount: u64, interest_rate: u64, due_date: i64) -> Self {
        Self {
            borrower,
            loan_amount,
            interest_rate,
            due_date,
            is_active: true,
        }
    }

    pub fn close(&mut self) {
        self.is_active = false;
    }
}
