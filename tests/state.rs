use super::*;
use anchor_lang::prelude::*;

#[test]
fn test_account_state() {
    let program = Program::new("solana_program", "FXWWRYqjwGTagnr8Gvs1zu6vviUcRMNVvNEjwszwKwun");
    let account = Account {
        owner: Pubkey::new_from_array([0; 32]),
        amount: 100,
    };
    let result = program.account(account);
    assert!(result.is_ok());
}