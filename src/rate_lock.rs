#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, Address, Env, Symbol};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RateLockError {
    NoRateLocked = 1,
    RateExpired = 2,
}

#[contract]
pub struct RateLockContract;

#[contractimpl]
impl RateLockContract {
    pub fn lock_rate(env: Env, user: Address, rate: i128, duration_seconds: u64) {
        let expiry = env.ledger().timestamp() + duration_seconds;
        let key = (user.clone(), symbol_short!("RATELOCK"));
        env.storage().persistent().set(&key, &(rate, expiry));
    }

    pub fn validate_conversion(env: Env, user: Address) -> Result<i128, RateLockError> {
        let key = (user.clone(), symbol_short!("RATELOCK"));
        let stored: Option<(i128, u64)> = env.storage().persistent().get(&key);

        let (rate, expiry) = stored.ok_or(RateLockError::NoRateLocked)?;

        if env.ledger().timestamp() > expiry {
            return Err(RateLockError::RateExpired);
        }

        Ok(rate)
    }
}
