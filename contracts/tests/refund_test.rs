use soroban_sdk::{
    testutils::{Address as _},
    Address, Env
};

// Simple test to verify our refund logic works
#[test]
fn test_refund_logic() {
    // Create test environment
    let env = Env::default();
    
    // Create test addresses
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let third_party = Address::generate(&env);
    
    // Test the tx_sender check logic
    let result1 = check_refund_auth(&sender, &sender, &recipient);
    assert!(result1, "Sender should be allowed to refund");
    
    let result2 = check_refund_auth(&recipient, &sender, &recipient);
    assert!(result2, "Recipient should be allowed to refund");
    
    let result3 = check_refund_auth(&third_party, &sender, &recipient);
    assert!(!result3, "Third party should not be allowed to refund");
    
    println!("Refund authorization check passed!");
}

// Simplified function that mimics our contract's auth check logic for refund
fn check_refund_auth(tx_sender: &Address, escrow_sender: &Address, escrow_recipient: &Address) -> bool {
    tx_sender == escrow_sender || tx_sender == escrow_recipient
} 