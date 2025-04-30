use soroban_sdk::Env;

#[test]
fn test_environment() {
    let env = Env::default();
    
    // Just test that we can create an environment
    let timestamp = env.ledger().timestamp();
    assert!(timestamp > 0 || timestamp == 0);
} 