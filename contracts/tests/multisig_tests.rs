use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, Address, Env, IntoVal,
};
use soroban_auth::{Signature, SignaturePayload};

mod contract {
    soroban_sdk::contractimport!(file = "../src/multisig.rs");
}

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, contract::MultiSigContract {});
    let client = contract::Client::new(&env, &contract_id);

    // Create test signers
    let signer1 = Address::random(&env);
    let signer2 = Address::random(&env);
    let signer3 = Address::random(&env);
    let signers = vec![&env, signer1.clone(), signer2.clone(), signer3.clone()];

    // Initialize with 2-of-3 configuration
    let config = client.initialize(&signers, &2);
    assert_eq!(config.threshold, 2);
    assert_eq!(config.signers.len(), 3);
    assert_eq!(config.nonce, 0);
}

#[test]
#[should_panic(expected = "Invalid threshold")]
fn test_initialize_invalid_threshold() {
    let env = Env::default();
    let contract_id = env.register_contract(None, contract::MultiSigContract {});
    let client = contract::Client::new(&env, &contract_id);

    // Try to initialize with invalid threshold
    let signers = vec![&env, Address::random(&env)];
    client.initialize(&signers, &2); // Should panic
}

#[test]
fn test_propose_transaction() {
    let env = Env::default();
    let contract_id = env.register_contract(None, contract::MultiSigContract {});
    let client = contract::Client::new(&env, &contract_id);

    // Create test signers
    let signer1 = Address::random(&env);
    let signer2 = Address::random(&env);
    let signer3 = Address::random(&env);
    let signers = vec![&env, signer1.clone(), signer2.clone(), signer3.clone()];

    // Initialize with 2-of-3 configuration
    client.initialize(&signers, &2);

    // Create test operation
    let operation = vec![&env, 1, 2, 3];
    let timestamp = env.ledger().timestamp();

    // Create signature payload
    let payload = SignaturePayload {
        contract_id: contract_id.clone(),
        network_id: env.ledger().network_id(),
        function_name: "propose_transaction".into(),
        args: (operation.clone(), timestamp, 0u32).into(),
    };

    // Sign with two signers
    let sig1 = Signature::sign_payload(&env, &signer1, &payload);
    let sig2 = Signature::sign_payload(&env, &signer2, &payload);
    let signatures = vec![&env, sig1, sig2];

    // Propose transaction
    let result = client.propose_transaction(&operation, &signatures);
    assert!(result);

    // Verify nonce increment
    let config = client.get_config();
    assert_eq!(config.nonce, 1);
}

#[test]
fn test_propose_transaction_insufficient_signatures() {
    let env = Env::default();
    let contract_id = env.register_contract(None, contract::MultiSigContract {});
    let client = contract::Client::new(&env, &contract_id);

    // Create test signers
    let signer1 = Address::random(&env);
    let signer2 = Address::random(&env);
    let signer3 = Address::random(&env);
    let signers = vec![&env, signer1.clone(), signer2.clone(), signer3.clone()];

    // Initialize with 2-of-3 configuration
    client.initialize(&signers, &2);

    // Create test operation
    let operation = vec![&env, 1, 2, 3];
    let timestamp = env.ledger().timestamp();

    // Create signature payload
    let payload = SignaturePayload {
        contract_id: contract_id.clone(),
        network_id: env.ledger().network_id(),
        function_name: "propose_transaction".into(),
        args: (operation.clone(), timestamp, 0u32).into(),
    };

    // Sign with only one signer
    let sig1 = Signature::sign_payload(&env, &signer1, &payload);
    let signatures = vec![&env, sig1];

    // Propose transaction should fail
    let result = client.propose_transaction(&operation, &signatures);
    assert!(!result);

    // Verify nonce remains unchanged
    let config = client.get_config();
    assert_eq!(config.nonce, 0);
}

#[test]
fn test_replay_protection() {
    let env = Env::default();
    let contract_id = env.register_contract(None, contract::MultiSigContract {});
    let client = contract::Client::new(&env, &contract_id);

    // Create test signers
    let signer1 = Address::random(&env);
    let signer2 = Address::random(&env);
    let signers = vec![&env, signer1.clone(), signer2.clone()];

    // Initialize with 2-of-2 configuration
    client.initialize(&signers, &2);

    // Create test operation
    let operation = vec![&env, 1, 2, 3];
    let timestamp = env.ledger().timestamp();

    // Create signature payload
    let payload = SignaturePayload {
        contract_id: contract_id.clone(),
        network_id: env.ledger().network_id(),
        function_name: "propose_transaction".into(),
        args: (operation.clone(), timestamp, 0u32).into(),
    };

    // Sign with both signers
    let sig1 = Signature::sign_payload(&env, &signer1, &payload);
    let sig2 = Signature::sign_payload(&env, &signer2, &payload);
    let signatures = vec![&env, sig1.clone(), sig2.clone()];

    // First proposal should succeed
    let result = client.propose_transaction(&operation, &signatures);
    assert!(result);

    // Attempt replay should fail due to nonce mismatch
    let result = client.propose_transaction(&operation, &signatures);
    assert!(!result);
}