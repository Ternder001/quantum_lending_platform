use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use solana_program_test::*;
use solana_sdk::signer::Signer;

#[tokio::test]
async fn test_deposit() {
    let program_test = ProgramTest::new("quantum_lending_platform", quantum_lending_platform::id(), processor!(quantum_lending_platform::entry));
    let mut context = program_test.start_with_context().await;

    // Simulate deposit call
    let result = quantum_lending_platform::deposit(
        context,
        Pubkey::default(),
        1000,
    ).await;

    assert!(result.is_ok());
}
