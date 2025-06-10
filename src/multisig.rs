#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, BytesN, Env, Symbol, Vec,
};

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

const CONFIG_KEY: Symbol = symbol_short!("CONFIG");

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
            },
        );
        crate::event::EventEmitter::emit_event(&env, crate::event::MULTISIG_TOPIC, event);

        if valid_signatures >= config.threshold {
            let exec_event = crate::event::DeFiEvent::MultisigTransactionExecuted(
                crate::event::MultisigTransactionExecutedData {
                    nonce: config.nonce,
                    signers: config.signers.clone(),
                    operation_hash: operation,
                    executed_at: env.ledger().timestamp(),
                },
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
            },
        );
        crate::event::EventEmitter::emit_event(&env, crate::event::MULTISIG_TOPIC, event);
        env.storage().instance().set(&CONFIG_KEY, &new_config);
        new_config
    }
}