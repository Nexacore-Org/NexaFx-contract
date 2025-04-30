use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(_env: Env, admin: Address) -> Address {
        admin
    }
    
    pub fn mint(_env: Env, _to: Address, _amount: i128) {
        // Just a stub for testing
    }
    
    pub fn transfer(_env: Env, _from: Address, _to: Address, _amount: i128) {
        // Just a stub for testing
    }
    
    pub fn balance(_env: Env, _of: Address) -> i128 {
        // For testing, return a simple value based on the address
        100
    }
} 