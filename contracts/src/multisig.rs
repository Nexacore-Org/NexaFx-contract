use soroban_sdk::{contract, contractimpl, contracttype, vec, Address, Env, Vec};
use soroban_auth::{Signature, SignaturePayload};

#[contracttype]
pub struct MultiSigConfig {
    signers: Vec<Address>,
    threshold: u32,
    nonce: u32,
}

#[contracttype]
pub struct Transaction {
    operation: Vec<u8>,
    timestamp: u64,
    nonce: u32,
}

#[contract]
pub struct MultiSigContract;

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

        env.storage().instance().set(&config);
        config
    }

    pub fn propose_transaction(
        env: Env,
        operation: Vec<u8>,
        signatures: Vec<Signature>,
    ) -> bool {
        let config: MultiSigConfig = env.storage().instance().get().unwrap();
        let timestamp = env.ledger().timestamp();

        let transaction = Transaction {
            operation: operation.clone(),
            timestamp,
            nonce: config.nonce,
        };

        // Create payload for signature verification
        let payload = SignaturePayload {
            contract_id: env.current_contract_id(),
            network_id: env.ledger().network_id(),
            function_name: "propose_transaction".into(),
            args: (operation, timestamp, config.nonce).into(),
        };

        // Verify signatures
        let mut valid_signatures = 0;
        let mut used_signers = vec![&env];

        for signature in signatures.iter() {
            let signer = signature.verify(&payload);
            
            // Check if signer is authorized
            if config.signers.contains(&signer) && !used_signers.contains(&signer) {
                valid_signatures += 1;
                used_signers.push_back(signer);
            }
        }

        // Check if threshold is met
        if valid_signatures >= config.threshold {
            // Update nonce for replay protection
            let new_config = MultiSigConfig {
                signers: config.signers,
                threshold: config.threshold,
                nonce: config.nonce + 1,
            };
            env.storage().instance().set(&new_config);

            // Execute the operation
            // Note: In a real implementation, you would decode and execute the operation here
            true
        } else {
            false
        }
    }

    pub fn get_config(env: Env) -> MultiSigConfig {
        env.storage().instance().get().unwrap()
    }
}