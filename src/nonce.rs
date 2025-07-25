#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, Address, Env, Symbol};

#[contracterror]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ContractError {
    InvalidNonce = 1,
}

#[contract]
pub struct NonceTracker;

#[contractimpl]
impl NonceTracker {
    pub fn get_nonce(env: Env, user: Address) -> u64 {
        let key = (user.clone(), symbol_short!("NONCE"));
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    pub fn check_and_update_nonce(
        env: Env,
        user: Address,
        incoming: u64,
    ) -> Result<u64, ContractError> {
        let key = (user.clone(), symbol_short!("NONCE"));
        let stored: u64 = env.storage().persistent().get(&key).unwrap_or(0);

        if incoming <= stored {
            return Err(ContractError::InvalidNonce);
        }

        env.storage().persistent().set(&key, &incoming);
        Ok(incoming)
    }
}
