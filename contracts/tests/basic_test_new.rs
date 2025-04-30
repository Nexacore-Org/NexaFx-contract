use soroban_sdk::{
    Env, Symbol, String
};

// This is a basic sanity test to make sure the environment is working
#[test]
fn test_basic_operations() {
    let env = Env::default();
    
    // Test creating a symbol
    let symbol = Symbol::new(&env, "test_symbol");
    assert_eq!(symbol.to_string(), "test_symbol");
    
    // Test creating a string
    let hello = String::from_str(&env, "Hello");
    let world = String::from_str(&env, "World");
    assert_eq!(hello.to_string(), "Hello");
    assert_ne!(hello.to_string(), world.to_string());
} 