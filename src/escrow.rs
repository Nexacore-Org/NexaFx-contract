use core::fmt::Write;
use heapless::String as HString;
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, Env, Symbol, Vec,
};

/// Status of the escrow operation
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowStatus {
    /// Escrow is active and funds are locked
    Active,
    /// Funds have been released to the recipient
    Released,
    /// Funds have been returned to the sender
    Refunded,
    /// Funds were automatically released after timeout
    AutoReleased,
}

/// Configuration for the escrow
#[contracttype]
#[derive(Clone)]
pub struct EscrowConfig {
    /// The escrow identifier
    id: Symbol,
    /// Address of the user sending the funds
    sender: Address,
    /// Address of the user receiving the funds
    recipient: Address,
    /// Token being held in escrow
    token: Address,
    /// Amount of tokens in escrow
    amount: i128,
    /// Timestamp when the escrow was created
    created_at: u64,
    /// Timeout period in seconds after which funds auto-release
    timeout_duration: u64,
    /// Current status of the escrow
    status: EscrowStatus,
}

/// Public information about an escrow
#[contracttype]
pub struct EscrowInfo {
    id: Symbol,
    sender: Address,
    recipient: Address,
    token: Address,
    amount: i128,
    created_at: u64,
    timeout_at: u64,
    status: EscrowStatus,
}

#[contract]
pub struct EscrowContract;

const ESCROW_COUNT_KEY: Symbol = symbol_short!("CNT");

#[contractimpl]
impl EscrowContract {
    /// Create a new escrow
    pub fn create(
        env: Env,
        sender: Address,
        recipient: Address,
        token: Address,
        amount: i128,
        timeout_duration: u64,
    ) -> EscrowInfo {
        // Validate inputs
        if amount <= 0 {
            panic!("Amount must be positive");
        }
        if timeout_duration == 0 {
            panic!("Timeout duration must be non-zero");
        }

        // Authenticate the sender
        sender.require_auth();

        // Transfer tokens from sender to contract
        let client = token::Client::new(&env, &token);
        client.transfer(&sender, &env.current_contract_address(), &amount);

        // Create a unique ID for this escrow
        let count = env
            .storage()
            .instance()
            .get(&ESCROW_COUNT_KEY)
            .unwrap_or(0u32);
        let mut s: HString<12> = HString::new();
        s.push_str("escrow_").unwrap();
        write!(&mut s, "{}", count).unwrap();
        let id = Symbol::new(&env, s.as_str());
        env.storage()
            .instance()
            .set(&ESCROW_COUNT_KEY, &(count + 1));

        // Get current timestamp
        let created_at = env.ledger().timestamp();

        // Store escrow configuration
        let escrow = EscrowConfig {
            id: id.clone(),
            sender: sender.clone(),
            recipient: recipient.clone(),
            token: token.clone(),
            amount,
            created_at,
            timeout_duration,
            status: EscrowStatus::Active,
        };

        crate::event::EventEmitter::emit_escrow_created(
            &env,
            id.clone(),
            sender.clone(),
            recipient.clone(),
            token.clone(),
            amount,
            timeout_duration,
        );

        // Save the escrow
        env.storage().instance().set(&id, &escrow);

        // Return escrow info
        EscrowInfo {
            id,
            sender,
            recipient,
            token,
            amount,
            created_at,
            timeout_at: created_at + timeout_duration,
            status: EscrowStatus::Active,
        }
    }

    /// Release funds to the recipient (can only be called by sender)
    pub fn release(env: Env, escrow_id: Symbol) -> EscrowInfo {
        // Get the escrow
        let escrow: EscrowConfig = env.storage().instance().get(&escrow_id).unwrap();

        // Validate the escrow is active
        if escrow.status != EscrowStatus::Active {
            panic!("Escrow is not active");
        }

        // Require sender authorization
        escrow.sender.require_auth();

        // Transfer the tokens to the recipient
        let client = token::Client::new(&env, &escrow.token);
        client.transfer(
            &env.current_contract_address(),
            &escrow.recipient,
            &escrow.amount,
        );

        // Emit escrow release event
        crate::event::EventEmitter::emit_escrow_released(
            &env,
            escrow_id.clone(),
            escrow.sender.clone(),
            escrow.recipient.clone(),
            escrow.token.clone(),
            escrow.amount,
        );

        // Update the escrow status
        let updated_escrow = EscrowConfig {
            status: EscrowStatus::Released,
            ..escrow.clone()
        };
        env.storage().instance().set(&escrow_id, &updated_escrow);

        // Return updated escrow info
        EscrowInfo {
            id: escrow.id,
            sender: escrow.sender,
            recipient: escrow.recipient,
            token: escrow.token,
            amount: escrow.amount,
            created_at: escrow.created_at,
            timeout_at: escrow.created_at + escrow.timeout_duration,
            status: EscrowStatus::Released,
        }
    }

    /// Refund the tokens back to the sender (can be called by both sender and recipient)
    pub fn refund(env: Env, escrow_id: Symbol) -> EscrowInfo {
        // Get the escrow
        let escrow: EscrowConfig = env.storage().instance().get(&escrow_id).unwrap();

        // Validate the escrow is active
        if escrow.status != EscrowStatus::Active {
            panic!("Escrow is not active");
        }

        // For now, we'll just require the sender to authenticate for refund
        // This is a simplification but ensures security
        escrow.sender.require_auth();

        // Transfer the tokens back to the sender
        let client = token::Client::new(&env, &escrow.token);
        client.transfer(
            &env.current_contract_address(),
            &escrow.sender,
            &escrow.amount,
        );

        // Update the escrow status
        let updated_escrow = EscrowConfig {
            status: EscrowStatus::Refunded,
            ..escrow.clone()
        };
        env.storage().instance().set(&escrow_id, &updated_escrow);

        // Return updated escrow info
        EscrowInfo {
            id: escrow.id,
            sender: escrow.sender,
            recipient: escrow.recipient,
            token: escrow.token,
            amount: escrow.amount,
            created_at: escrow.created_at,
            timeout_at: escrow.created_at + escrow.timeout_duration,
            status: EscrowStatus::Refunded,
        }
    }

    /// Check if the escrow has timed out and release funds if necessary
    pub fn check_timeout(env: Env, escrow_id: Symbol) -> EscrowInfo {
        // Get the escrow
        let escrow: EscrowConfig = env.storage().instance().get(&escrow_id).unwrap();

        // Validate the escrow is active
        if escrow.status != EscrowStatus::Active {
            panic!("Escrow is not active");
        }

        // Check if timeout has been reached
        let current_time = env.ledger().timestamp();
        let timeout_time = escrow.created_at + escrow.timeout_duration;

        if current_time < timeout_time {
            panic!("Escrow has not timed out yet");
        }

        // Transfer the tokens to the recipient (auto-release)
        let client = token::Client::new(&env, &escrow.token);
        client.transfer(
            &env.current_contract_address(),
            &escrow.recipient,
            &escrow.amount,
        );

        // Update the escrow status
        let updated_escrow = EscrowConfig {
            status: EscrowStatus::AutoReleased,
            ..escrow.clone()
        };
        env.storage().instance().set(&escrow_id, &updated_escrow);

        // Return updated escrow info
        EscrowInfo {
            id: escrow.id,
            sender: escrow.sender,
            recipient: escrow.recipient,
            token: escrow.token,
            amount: escrow.amount,
            created_at: escrow.created_at,
            timeout_at: timeout_time,
            status: EscrowStatus::AutoReleased,
        }
    }

    /// Get information about an escrow
    pub fn get_escrow(env: Env, escrow_id: Symbol) -> EscrowInfo {
        // Get the escrow
        let escrow: EscrowConfig = env.storage().instance().get(&escrow_id).unwrap();

        // Return escrow info
        EscrowInfo {
            id: escrow.id,
            sender: escrow.sender,
            recipient: escrow.recipient,
            token: escrow.token,
            amount: escrow.amount,
            created_at: escrow.created_at,
            timeout_at: escrow.created_at + escrow.timeout_duration,
            status: escrow.status,
        }
    }

    /// Get all active escrows
    pub fn get_all_escrows(env: Env) -> Vec<EscrowInfo> {
        let count = env
            .storage()
            .instance()
            .get(&ESCROW_COUNT_KEY)
            .unwrap_or(0u32);
        let mut escrows = Vec::new(&env);

        for i in 0..count {
            let mut s: HString<12> = HString::new();
            s.push_str("escrow_").unwrap();
            write!(&mut s, "{}", i).unwrap();
            let id = Symbol::new(&env, s.as_str());
            if env.storage().instance().has(&id) {
                let escrow: EscrowConfig = env.storage().instance().get(&id).unwrap();
                escrows.push_back(EscrowInfo {
                    id: escrow.id,
                    sender: escrow.sender,
                    recipient: escrow.recipient,
                    token: escrow.token,
                    amount: escrow.amount,
                    created_at: escrow.created_at,
                    timeout_at: escrow.created_at + escrow.timeout_duration,
                    status: escrow.status,
                });
            }
        }

        escrows
    }
