#![cfg(test)]

use soroban_sdk::{Env, Address};
use soroban_sdk::testutils::Address as _;
use stellar_multisig_contract::nonce::NonceTracker;
use stellar_multisig_contract::nonce::ContractError;

#[test]
fn test_nonce_tracker() {
        let env = Env::default();
        let contract_id = env.register(NonceTracker, ());
        let user = Address::generate(&env);

        env.as_contract(&contract_id, || {
        assert_eq!(NonceTracker::get_nonce(env.clone(), user.clone()), 0);
        NonceTracker::check_and_update_nonce(env.clone(), user.clone(), 1).unwrap();
        assert_eq!(NonceTracker::get_nonce(env.clone(), user.clone()), 1);
        });

        env.as_contract(&contract_id, || {
        let err = NonceTracker::check_and_update_nonce(env.clone(), user.clone(), 1).unwrap_err();
        assert_eq!(err, ContractError::InvalidNonce);
        NonceTracker::check_and_update_nonce(env.clone(), user.clone(), 2).unwrap();
        assert_eq!(NonceTracker::get_nonce(env.clone(), user.clone()), 2);
    })
}