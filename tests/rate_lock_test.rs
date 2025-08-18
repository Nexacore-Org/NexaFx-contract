#![cfg(test)]

use soroban_sdk::testutils::Address as _;
use soroban_sdk::testutils::Ledger;
use soroban_sdk::{Address, Env};
use stellar_multisig_contract::rate_lock::RateLockContractClient as RateLockClient;
use stellar_multisig_contract::rate_lock::{RateLockContract, RateLockError};

#[test]
fn test_lock_and_validate_rate() {
    let env = Env::default();
    let user = Address::generate(&env);
    let contract_id = env.register(RateLockContract, ());

    // Lock rate
    env.as_contract(&contract_id, || {
        RateLockContract::lock_rate(env.clone(), user.clone(), 100, 60);
    });

    // Validate inside contract context
    let rate = env.as_contract(&contract_id, || {
        RateLockContract::validate_conversion(env.clone(), user.clone()).unwrap()
    });
    assert_eq!(rate, 100);

    // Advance time
    env.ledger().set_timestamp(env.ledger().timestamp() + 61);

    let err = env.as_contract(&contract_id, || {
        RateLockContract::validate_conversion(env.clone(), user.clone()).unwrap_err()
    });
    assert_eq!(err, RateLockError::RateExpired);
}
