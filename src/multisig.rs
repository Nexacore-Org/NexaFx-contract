use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, BytesN, Symbol, Val};

#[contracttype]
#[derive(Clone, Debug)]
pub struct MultiSigConfig {
    signers: Vec<Address>,
    threshold: u32,
    nonce: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct Transaction {
    operation: BytesN<32>,
    timestamp: u64,
    nonce: u32,
}

#[contract]
pub struct MultiSigContract;

const CONFIG_KEY: Symbol = Symbol::short("CONFIG");

#[contractimpl]
impl MultiSigContract {
    pub fn initialize(env: Env, signers: Vec<Address>, threshold: u32) -> MultiSigConfig {
        if threshold == 0 || threshold > signers.len() as u32 {
            panic!("Invalid threshold");
        }

        let config = MultiSigConfig {
            signers,
            threshold,
            nonce: 0,
        };

        env.storage().instance().set(&CONFIG_KEY, &config);
        config
    }

    pub fn propose_transaction(
        env: Env,
        operation: BytesN<32>,
        signatures: Vec<BytesN<64>>,
        proposer: Address,
    ) -> bool {
        let mut config: MultiSigConfig = env.storage().instance().get(&CONFIG_KEY).unwrap();
        let timestamp = env.ledger().timestamp();

        let _transaction = Transaction {
            operation: operation.clone(),
            timestamp,
            nonce: config.nonce,
        };

        let valid_signatures = signatures.len() as u32;

        let event = crate::event::DeFiEvent::MultisigTransactionProposed(
            crate::event::MultisigTransactionProposedData {
                nonce: config.nonce,
                proposer: proposer.clone(),
                operation_hash: operation.clone(),
                threshold: config.threshold,
                current_signatures: valid_signatures,
                proposed_at: env.ledger().timestamp(),
            }
        );
        crate::event::EventEmitter::emit_event(&env, crate::event::MULTISIG_TOPIC, event);

        if valid_signatures >= config.threshold {
            let exec_event = crate::event::DeFiEvent::MultisigTransactionExecuted(
                crate::event::MultisigTransactionExecutedData {
                    nonce: config.nonce,
                    signers: config.signers.clone(),
                    operation_hash: operation,
                    executed_at: env.ledger().timestamp(),
                }
            );
            crate::event::EventEmitter::emit_event(&env, crate::event::MULTISIG_TOPIC, exec_event);
            
            config.nonce += 1;
            env.storage().instance().set(&CONFIG_KEY, &config);
            
            true
        } else {
            false
        }
    }

    pub fn get_config(env: Env) -> MultiSigConfig {
        env.storage().instance().get(&CONFIG_KEY).unwrap()
    }

    pub fn update_config(
        env: Env,
        new_signers: Vec<Address>,
        new_threshold: u32,
        proposer: Address,
    ) -> MultiSigConfig {
        if new_threshold == 0 || new_threshold > new_signers.len() as u32 {
            panic!("Invalid threshold");
        }

        let old_config: MultiSigConfig = env.storage().instance().get(&CONFIG_KEY).unwrap();
        
        let new_config = MultiSigConfig {
            signers: new_signers.clone(),
            threshold: new_threshold,
            nonce: old_config.nonce,
        };

        let event = crate::event::DeFiEvent::MultisigConfigUpdated(
            crate::event::MultisigConfigUpdatedData {
                old_signers: old_config.signers,
                new_signers,
                old_threshold: old_config.threshold,
                new_threshold,
                updated_at: env.ledger().timestamp(),
            }
        );
        crate::event::EventEmitter::emit_event(&env, crate::event::MULTISIG_TOPIC, event);

        env.storage().instance().set(&CONFIG_KEY, &new_config);
        new_config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::{Address as _, Events}, Address, Symbol, Vec, Val, unwrap::Unwrap};

    #[test]
    fn test_multisig_initialization() {
        let env = Env::default();
        env.mock_all_auths();

        let signer1 = Address::generate(&env);
        let signer2 = Address::generate(&env);
        let signers = Vec::from_array(&env, [signer1, signer2]);

        let config = MultiSigContract::initialize(env, signers.clone(), 2);

        assert_eq!(config.signers, signers);
        assert_eq!(config.threshold, 2);
        assert_eq!(config.nonce, 0);
    }

    #[test]
    fn test_propose_transaction() {
        let env = Env::default();
        env.mock_all_auths();

        let signer1 = Address::generate(&env);
        let signer2 = Address::generate(&env);
        let proposer = Address::generate(&env);
        let signers = Vec::from_array(&env, [signer1, signer2]);

        MultiSigContract::initialize(env.clone(), signers, 2);

        let operation = BytesN::from_array(&env, &[1u8; 32]);
        let sig1 = BytesN::from_array(&env, &[1u8; 64]);
        let sig2 = BytesN::from_array(&env, &[2u8; 64]);
        let signatures = Vec::from_array(&env, [sig1, sig2]);

        let result = MultiSigContract::propose_transaction(
            env.clone(),
            operation,
            signatures,
            proposer,
        );

        assert!(result);

        let events = env.events().all();
        assert_eq!(events.len(), 2); // Proposal + Execution events

        // Check proposal event
        let event = events.get(0).unwrap();
        let topics: Vec<Val> = event.1.clone();
        let data: Val = event.2.clone();
        let topic_symbol: Symbol = topics.get(0).unwrap().try_into().unwrap();
        let event_data: crate::event::DeFiEvent = data.unwrap();

        assert_eq!(topic_symbol, crate::event::MULTISIG_EVENT);
        match event_data {
            crate::event::DeFiEvent::MultisigTransactionProposed(_) => {},
            _ => panic!("Wrong event type emitted for proposal"),
        }

        // Check execution event
        let event = events.get(1).unwrap();
        let topics: Vec<Val> = event.1.clone();
        let data: Val = event.2.clone();
        let topic_symbol: Symbol = topics.get(0).unwrap().try_into().unwrap();
        let event_data: crate::event::DeFiEvent = data.unwrap();

        assert_eq!(topic_symbol, crate::event::MULTISIG_EVENT);
        match event_data {
            crate::event::DeFiEvent::MultisigTransactionExecuted(_) => {},
            _ => panic!("Wrong event type emitted for execution"),
        }
    }

    #[test]
    fn test_insufficient_signatures() {
        let env = Env::default();
        env.mock_all_auths();

        let signer1 = Address::generate(&env);
        let signer2 = Address::generate(&env);
        let proposer = Address::generate(&env);
        let signers = Vec::from_array(&env, [signer1, signer2]);

        MultiSigContract::initialize(env.clone(), signers, 2);

        let operation = BytesN::from_array(&env, &[1u8; 32]);
        let sig1 = BytesN::from_array(&env, &[1u8; 64]);
        let signatures = Vec::from_array(&env, [sig1]);

        let result = MultiSigContract::propose_transaction(
            env.clone(),
            operation,
            signatures,
            proposer,
        );

        assert!(!result);

        let events = env.events().all();
        assert_eq!(events.len(), 1); // Only proposal event

        let event = events.get(0).unwrap();
        let topics: Vec<Val> = event.1.clone();
        let data: Val = event.2.clone();
        let topic_symbol: Symbol = topics.get(0).unwrap().try_into().unwrap();
        let event_data: crate::event::DeFiEvent = data.unwrap();

        assert_eq!(topic_symbol, crate::event::MULTISIG_EVENT);
        match event_data {
            crate::event::DeFiEvent::MultisigTransactionProposed(_) => {},
            _ => panic!("Wrong event type emitted"),
        }
    }
}

#[cfg(not(test))]
fn main() {
    let env = Env::default();

    let signer1 = Address::from_string(&soroban_sdk::String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"));
    let signer2 = Address::from_string(&soroban_sdk::String::from_str(&env, "GBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB"));
    let signers = Vec::from_array(&env, [signer1, signer2]);

    let multisig = MultiSigContract::initialize(env, signers, 1);

    println!("MultiSig initialized: {:?}", multisig);
}