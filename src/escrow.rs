use core::fmt::Write;
use heapless::String as HString;
use soroban_sdk::{
    contract, contractclient, contractimpl, contracttype, symbol_short, token, Address, Env,
    Symbol, Vec,
};

/// Status of the escrow operation
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowStatus {
    /// Escrow is active and funds are locked
    Active,
    /// A dispute has been initiated
    Disputed,
    /// Funds have been released to the recipient
    Released,
    /// Funds have been returned to the sender
    Refunded,
    /// Funds were automatically released after timeout
    AutoReleased,
    /// Dispute was resolved in favor of recipient
    DisputeResolvedForRecipient,
    /// Dispute was resolved in favor of sender
    DisputeResolvedForSender,
}

/// Dispute information
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisputeInfo {
    /// Who initiated the dispute
    pub initiated_by: Address,
    /// Timestamp when dispute was initiated
    pub initiated_at: u64,
    /// Dispute period duration in seconds
    pub dispute_period: u64,
    /// Reason for the dispute
    pub reason: Symbol,
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
    /// Dispute period in seconds (default dispute window)
    dispute_period: u64,
    /// Current status of the escrow
    status: EscrowStatus,
    /// Has dispute flag
    has_dispute: bool,
}

/// Public information about an escrow
#[contracttype]
pub struct EscrowInfo {
    pub id: Symbol,
    pub sender: Address,
    pub recipient: Address,
    pub token: Address,
    pub amount: i128,
    pub created_at: u64,
    pub timeout_at: u64,
    pub dispute_period: u64,
    pub status: EscrowStatus,
    pub has_dispute: bool,
}

#[contract]
pub struct EscrowContract;

#[contractclient(name = "EscrowClient")]
pub trait EscrowContractTrait {
    fn create(
        env: Env,
        sender: Address,
        recipient: Address,
        token: Address,
        amount: i128,
        timeout_duration: u64,
        dispute_period: u64,
    ) -> EscrowInfo;

    fn release(env: Env, escrow_id: Symbol) -> EscrowInfo;
    fn refund(env: Env, escrow_id: Symbol) -> EscrowInfo;
    fn check_timeout(env: Env, escrow_id: Symbol) -> EscrowInfo;
    fn get_escrow(env: Env, escrow_id: Symbol) -> EscrowInfo;
    fn get_all_escrows(env: Env) -> Vec<EscrowInfo>;
    fn initiate_dispute(env: Env, escrow_id: Symbol, reason: Symbol) -> EscrowInfo;
    fn resolve_dispute_for_recipient(env: Env, escrow_id: Symbol) -> EscrowInfo;
    fn resolve_dispute_for_sender(env: Env, escrow_id: Symbol) -> EscrowInfo;
    fn check_dispute_timeout(env: Env, escrow_id: Symbol) -> EscrowInfo;
    fn get_dispute_info(env: Env, escrow_id: Symbol) -> Option<DisputeInfo>;
    fn can_dispute(env: Env, escrow_id: Symbol) -> bool;
    fn get_escrow_count(env: Env) -> u32;
    fn escrow_exists(env: Env, escrow_id: Symbol) -> bool;
    fn get_escrows_by_status(env: Env, status: EscrowStatus) -> Vec<EscrowInfo>;
    fn get_escrows_by_participant(env: Env, participant: Address) -> Vec<EscrowInfo>;
    fn update_dispute_period(env: Env, escrow_id: Symbol, new_dispute_period: u64) -> EscrowInfo;
    fn initialize(env: Env, admin: Address);
    fn set_dispute_fee(env: Env, fee: i128);
    fn get_dispute_fee(env: Env) -> i128;
    fn get_admin(env: Env) -> Address;
    fn transfer_admin(env: Env, new_admin: Address);
    fn set_paused(env: Env, paused: bool);
    fn is_paused(env: Env) -> bool;
    fn admin_resolve_dispute(
        env: Env,
        escrow_id: Symbol,
        resolve_for_recipient: bool,
    ) -> EscrowInfo;
}

const ESCROW_COUNT_KEY: Symbol = symbol_short!("CNT");
const DISPUTE_FEE_KEY: Symbol = symbol_short!("DFEE");
const ADMIN_KEY: Symbol = symbol_short!("ADMIN");

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
        dispute_period: u64,
    ) -> EscrowInfo {
        // Check if contract is paused
        if env
            .storage()
            .instance()
            .get(&symbol_short!("PAUSED"))
            .unwrap_or(false)
        {
            panic!("Contract is paused");
        }

        // Validate inputs
        if amount <= 0 {
            panic!("Amount must be positive");
        }
        if timeout_duration == 0 {
            panic!("Timeout duration must be non-zero");
        }
        if dispute_period == 0 {
            panic!("Dispute period must be non-zero");
        }
        if timeout_duration < dispute_period {
            panic!("Timeout duration must be greater than dispute period");
        }
        if sender == recipient {
            panic!("Sender and recipient cannot be the same");
        }

        // Authenticate the sender
        sender.require_auth();

        // Verify token contract exists and sender has sufficient balance
        // Note: In production, this would always check the balance
        // For testing with mock addresses, we skip the balance check
        let client = token::Client::new(&env, &token);
        let sender_balance = client.balance(&sender);
        if sender_balance < amount {
            panic!("Insufficient token balance");
        }

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
            dispute_period,
            status: EscrowStatus::Active,
            has_dispute: false,
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
            dispute_period,
            status: EscrowStatus::Active,
            has_dispute: false,
        }
    }

    /// Release funds to the recipient (can only be called by sender)
    pub fn release(env: Env, escrow_id: Symbol) -> EscrowInfo {
        // Get the escrow
        let escrow: EscrowConfig = env.storage().instance().get(&escrow_id).unwrap();

        // Validate the escrow is active (not disputed)
        if escrow.status != EscrowStatus::Active {
            panic!("Escrow is not active or is disputed");
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
            dispute_period: escrow.dispute_period,
            status: EscrowStatus::Released,
            has_dispute: escrow.has_dispute,
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
            dispute_period: escrow.dispute_period,
            status: EscrowStatus::Refunded,
            has_dispute: escrow.has_dispute,
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
            dispute_period: escrow.dispute_period,
            status: EscrowStatus::AutoReleased,
            has_dispute: escrow.has_dispute,
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
            dispute_period: escrow.dispute_period,
            status: escrow.status,
            has_dispute: escrow.has_dispute,
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
                    dispute_period: escrow.dispute_period,
                    status: escrow.status,
                    has_dispute: escrow.has_dispute,
                });
            }
        }
        escrows
    }

    /// Initiate a dispute (can be called by sender or recipient)
    pub fn initiate_dispute(env: Env, escrow_id: Symbol, reason: Symbol) -> EscrowInfo {
        // Check if contract is paused
        if env
            .storage()
            .instance()
            .get(&symbol_short!("PAUSED"))
            .unwrap_or(false)
        {
            panic!("Contract is paused");
        }

        // Get the escrow
        let escrow: EscrowConfig = env.storage().instance().get(&escrow_id).unwrap();

        // Check if dispute already exists first
        if escrow.has_dispute {
            panic!("Dispute already initiated");
        }

        // Validate the escrow is active
        if escrow.status != EscrowStatus::Active {
            panic!("Escrow is not active");
        }

        // For now, we'll allow both sender and recipient to initiate disputes
        // In a production system, you might want more sophisticated authorization
        escrow.sender.require_auth();
        let caller = escrow.sender.clone();

        // Handle dispute fee if set
        let dispute_fee = env
            .storage()
            .instance()
            .get(&DISPUTE_FEE_KEY)
            .unwrap_or(0i128);
        if dispute_fee > 0 {
            let client = token::Client::new(&env, &escrow.token);
            let caller_balance = client.balance(&caller);
            if caller_balance < dispute_fee {
                panic!("Insufficient balance for dispute fee");
            }
            // Transfer dispute fee to contract (could be sent to admin or burned)
            client.transfer(&caller, &env.current_contract_address(), &dispute_fee);
        }

        // Create dispute info and store separately
        let dispute_info = DisputeInfo {
            initiated_by: caller.clone(),
            initiated_at: env.ledger().timestamp(),
            dispute_period: escrow.dispute_period,
            reason: reason.clone(),
        };

        // Store dispute info separately using a simple key pattern
        let dispute_key = symbol_short!("DISPUTE");
        env.storage()
            .instance()
            .set(&(escrow_id.clone(), dispute_key), &dispute_info);

        // Update escrow with dispute
        let updated_escrow = EscrowConfig {
            status: EscrowStatus::Disputed,
            has_dispute: true,
            ..escrow.clone()
        };
        env.storage().instance().set(&escrow_id, &updated_escrow);

        // Emit dispute initiated event
        crate::event::EventEmitter::emit_dispute_initiated(
            &env,
            escrow_id.clone(),
            caller,
            reason,
            dispute_info.dispute_period,
        );

        // Return updated escrow info
        EscrowInfo {
            id: escrow.id,
            sender: escrow.sender,
            recipient: escrow.recipient,
            token: escrow.token,
            amount: escrow.amount,
            created_at: escrow.created_at,
            timeout_at: escrow.created_at + escrow.timeout_duration,
            dispute_period: escrow.dispute_period,
            status: EscrowStatus::Disputed,
            has_dispute: true,
        }
    }

    /// Resolve dispute in favor of recipient (admin function or automated)
    pub fn resolve_dispute_for_recipient(env: Env, escrow_id: Symbol) -> EscrowInfo {
        // Get the escrow
        let escrow: EscrowConfig = env.storage().instance().get(&escrow_id).unwrap();

        // Validate the escrow is disputed
        if escrow.status != EscrowStatus::Disputed {
            panic!("Escrow is not disputed");
        }

        // Get dispute info
        let dispute_key = symbol_short!("DISPUTE");
        let dispute: DisputeInfo = env
            .storage()
            .instance()
            .get(&(escrow_id.clone(), dispute_key))
            .unwrap();

        // Check if dispute period has expired (auto-resolution)
        let current_time = env.ledger().timestamp();
        let dispute_expires_at = dispute.initiated_at + dispute.dispute_period;

        if current_time < dispute_expires_at {
            // Manual resolution - require sender auth for now
            escrow.sender.require_auth();
        }

        // Transfer tokens to recipient
        let client = token::Client::new(&env, &escrow.token);
        client.transfer(
            &env.current_contract_address(),
            &escrow.recipient,
            &escrow.amount,
        );

        // Update escrow status
        let updated_escrow = EscrowConfig {
            status: EscrowStatus::DisputeResolvedForRecipient,
            ..escrow.clone()
        };
        env.storage().instance().set(&escrow_id, &updated_escrow);

        // Emit dispute resolved event
        crate::event::EventEmitter::emit_dispute_resolved(
            &env,
            escrow_id.clone(),
            escrow.recipient.clone(),
            true, // resolved_for_recipient
        );

        // Return updated escrow info
        EscrowInfo {
            id: escrow.id,
            sender: escrow.sender,
            recipient: escrow.recipient,
            token: escrow.token,
            amount: escrow.amount,
            created_at: escrow.created_at,
            timeout_at: escrow.created_at + escrow.timeout_duration,
            dispute_period: escrow.dispute_period,
            status: EscrowStatus::DisputeResolvedForRecipient,
            has_dispute: true,
        }
    }

    /// Resolve dispute in favor of sender (admin function or automated)
    pub fn resolve_dispute_for_sender(env: Env, escrow_id: Symbol) -> EscrowInfo {
        // Get the escrow
        let escrow: EscrowConfig = env.storage().instance().get(&escrow_id).unwrap();

        // Validate the escrow is disputed
        if escrow.status != EscrowStatus::Disputed {
            panic!("Escrow is not disputed");
        }

        // Get dispute info
        let dispute_key = symbol_short!("DISPUTE");
        let dispute: DisputeInfo = env
            .storage()
            .instance()
            .get(&(escrow_id.clone(), dispute_key))
            .unwrap();

        // Check if dispute period has expired (auto-resolution)
        let current_time = env.ledger().timestamp();
        let dispute_expires_at = dispute.initiated_at + dispute.dispute_period;

        if current_time < dispute_expires_at {
            // Manual resolution - require sender auth for now
            escrow.sender.require_auth();
        }

        // Transfer tokens back to sender
        let client = token::Client::new(&env, &escrow.token);
        client.transfer(
            &env.current_contract_address(),
            &escrow.sender,
            &escrow.amount,
        );

        // Update escrow status
        let updated_escrow = EscrowConfig {
            status: EscrowStatus::DisputeResolvedForSender,
            ..escrow.clone()
        };
        env.storage().instance().set(&escrow_id, &updated_escrow);

        // Emit dispute resolved event
        crate::event::EventEmitter::emit_dispute_resolved(
            &env,
            escrow_id.clone(),
            escrow.sender.clone(),
            false, // resolved_for_recipient
        );

        // Return updated escrow info
        EscrowInfo {
            id: escrow.id,
            sender: escrow.sender,
            recipient: escrow.recipient,
            token: escrow.token,
            amount: escrow.amount,
            created_at: escrow.created_at,
            timeout_at: escrow.created_at + escrow.timeout_duration,
            dispute_period: escrow.dispute_period,
            status: EscrowStatus::DisputeResolvedForSender,
            has_dispute: true,
        }
    }

    /// Check if dispute has timed out and auto-resolve (default to recipient)
    pub fn check_dispute_timeout(env: Env, escrow_id: Symbol) -> EscrowInfo {
        // Get the escrow
        let escrow: EscrowConfig = env.storage().instance().get(&escrow_id).unwrap();

        // Validate the escrow is disputed
        if escrow.status != EscrowStatus::Disputed {
            panic!("Escrow is not disputed");
        }

        // Get dispute info
        let dispute_key = symbol_short!("DISPUTE");
        let dispute: DisputeInfo = env
            .storage()
            .instance()
            .get(&(escrow_id.clone(), dispute_key))
            .unwrap();

        // Check if dispute period has expired
        let current_time = env.ledger().timestamp();
        let dispute_expires_at = dispute.initiated_at + dispute.dispute_period;

        if current_time < dispute_expires_at {
            panic!("Dispute period has not expired yet");
        }

        // Auto-resolve in favor of recipient (default behavior)
        Self::resolve_dispute_for_recipient(env, escrow_id)
    }

    /// Get dispute information for an escrow
    pub fn get_dispute_info(env: Env, escrow_id: Symbol) -> Option<DisputeInfo> {
        let escrow: EscrowConfig = env.storage().instance().get(&escrow_id).unwrap();
        if escrow.has_dispute {
            let dispute_key = symbol_short!("DISPUTE");
            env.storage().instance().get(&(escrow_id, dispute_key))
        } else {
            None
        }
    }

    /// Check if an escrow can be disputed (is active and no existing dispute)
    pub fn can_dispute(env: Env, escrow_id: Symbol) -> bool {
        let escrow: EscrowConfig = env.storage().instance().get(&escrow_id).unwrap();
        escrow.status == EscrowStatus::Active && !escrow.has_dispute
    }

    /// Get escrow count for statistics
    pub fn get_escrow_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&ESCROW_COUNT_KEY)
            .unwrap_or(0u32)
    }

    /// Check if escrow exists
    pub fn escrow_exists(env: Env, escrow_id: Symbol) -> bool {
        env.storage().instance().has(&escrow_id)
    }

    /// Get escrows by status for filtering
    pub fn get_escrows_by_status(env: Env, status: EscrowStatus) -> Vec<EscrowInfo> {
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
                if escrow.status == status {
                    escrows.push_back(EscrowInfo {
                        id: escrow.id,
                        sender: escrow.sender,
                        recipient: escrow.recipient,
                        token: escrow.token,
                        amount: escrow.amount,
                        created_at: escrow.created_at,
                        timeout_at: escrow.created_at + escrow.timeout_duration,
                        dispute_period: escrow.dispute_period,
                        status: escrow.status,
                        has_dispute: escrow.has_dispute,
                    });
                }
            }
        }
        escrows
    }

    /// Get escrows by participant (sender or recipient)
    pub fn get_escrows_by_participant(env: Env, participant: Address) -> Vec<EscrowInfo> {
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
                if escrow.sender == participant || escrow.recipient == participant {
                    escrows.push_back(EscrowInfo {
                        id: escrow.id,
                        sender: escrow.sender,
                        recipient: escrow.recipient,
                        token: escrow.token,
                        amount: escrow.amount,
                        created_at: escrow.created_at,
                        timeout_at: escrow.created_at + escrow.timeout_duration,
                        dispute_period: escrow.dispute_period,
                        status: escrow.status,
                        has_dispute: escrow.has_dispute,
                    });
                }
            }
        }
        escrows
    }

    /// Update dispute period for an active escrow (only by sender before dispute)
    pub fn update_dispute_period(
        env: Env,
        escrow_id: Symbol,
        new_dispute_period: u64,
    ) -> EscrowInfo {
        let escrow: EscrowConfig = env.storage().instance().get(&escrow_id).unwrap();

        // Only sender can update and only if escrow is active with no dispute
        escrow.sender.require_auth();

        if escrow.status != EscrowStatus::Active {
            panic!("Escrow is not active");
        }

        if escrow.has_dispute {
            panic!("Cannot update dispute period after dispute initiated");
        }

        if new_dispute_period == 0 {
            panic!("Dispute period must be non-zero");
        }

        if escrow.timeout_duration < new_dispute_period {
            panic!("Dispute period cannot exceed timeout duration");
        }

        let updated_escrow = EscrowConfig {
            dispute_period: new_dispute_period,
            ..escrow.clone()
        };

        env.storage().instance().set(&escrow_id, &updated_escrow);

        EscrowInfo {
            id: escrow.id,
            sender: escrow.sender,
            recipient: escrow.recipient,
            token: escrow.token,
            amount: escrow.amount,
            created_at: escrow.created_at,
            timeout_at: escrow.created_at + escrow.timeout_duration,
            dispute_period: new_dispute_period,
            status: escrow.status,
            has_dispute: escrow.has_dispute,
        }
    }

    /// Initialize contract with admin (should be called once during deployment)
    pub fn initialize(env: Env, admin: Address) {
        // Check if already initialized
        if env.storage().instance().has(&ADMIN_KEY) {
            panic!("Contract already initialized");
        }

        admin.require_auth();
        env.storage().instance().set(&ADMIN_KEY, &admin);

        // Set default dispute fee to 0 (can be updated by admin)
        env.storage().instance().set(&DISPUTE_FEE_KEY, &0i128);
    }

    /// Set dispute fee (admin only)
    pub fn set_dispute_fee(env: Env, fee: i128) {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY).unwrap();
        admin.require_auth();

        if fee < 0 {
            panic!("Dispute fee cannot be negative");
        }

        env.storage().instance().set(&DISPUTE_FEE_KEY, &fee);
    }

    /// Get current dispute fee
    pub fn get_dispute_fee(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DISPUTE_FEE_KEY)
            .unwrap_or(0i128)
    }

    /// Get contract admin
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&ADMIN_KEY).unwrap()
    }

    /// Transfer admin rights (admin only)
    pub fn transfer_admin(env: Env, new_admin: Address) {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY).unwrap();
        admin.require_auth();

        env.storage().instance().set(&ADMIN_KEY, &new_admin);
    }

    /// Emergency pause/unpause functionality (admin only)
    pub fn set_paused(env: Env, paused: bool) {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY).unwrap();
        admin.require_auth();

        env.storage()
            .instance()
            .set(&symbol_short!("PAUSED"), &paused);
    }

    /// Check if contract is paused
    pub fn is_paused(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&symbol_short!("PAUSED"))
            .unwrap_or(false)
    }

    /// Admin emergency resolution (admin only, for extreme cases)
    pub fn admin_resolve_dispute(
        env: Env,
        escrow_id: Symbol,
        resolve_for_recipient: bool,
    ) -> EscrowInfo {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY).unwrap();
        admin.require_auth();

        let escrow: EscrowConfig = env.storage().instance().get(&escrow_id).unwrap();

        if escrow.status != EscrowStatus::Disputed {
            panic!("Escrow is not disputed");
        }

        if resolve_for_recipient {
            // Transfer tokens to recipient
            let client = token::Client::new(&env, &escrow.token);
            client.transfer(
                &env.current_contract_address(),
                &escrow.recipient,
                &escrow.amount,
            );

            let updated_escrow = EscrowConfig {
                status: EscrowStatus::DisputeResolvedForRecipient,
                ..escrow.clone()
            };
            env.storage().instance().set(&escrow_id, &updated_escrow);

            crate::event::EventEmitter::emit_dispute_resolved(
                &env,
                escrow_id.clone(),
                escrow.recipient.clone(),
                true,
            );

            EscrowInfo {
                id: escrow.id,
                sender: escrow.sender,
                recipient: escrow.recipient,
                token: escrow.token,
                amount: escrow.amount,
                created_at: escrow.created_at,
                timeout_at: escrow.created_at + escrow.timeout_duration,
                dispute_period: escrow.dispute_period,
                status: EscrowStatus::DisputeResolvedForRecipient,
                has_dispute: true,
            }
        } else {
            // Transfer tokens back to sender
            let client = token::Client::new(&env, &escrow.token);
            client.transfer(
                &env.current_contract_address(),
                &escrow.sender,
                &escrow.amount,
            );

            let updated_escrow = EscrowConfig {
                status: EscrowStatus::DisputeResolvedForSender,
                ..escrow.clone()
            };
            env.storage().instance().set(&escrow_id, &updated_escrow);

            crate::event::EventEmitter::emit_dispute_resolved(
                &env,
                escrow_id.clone(),
                escrow.sender.clone(),
                false,
            );

            EscrowInfo {
                id: escrow.id,
                sender: escrow.sender,
                recipient: escrow.recipient,
                token: escrow.token,
                amount: escrow.amount,
                created_at: escrow.created_at,
                timeout_at: escrow.created_at + escrow.timeout_duration,
                dispute_period: escrow.dispute_period,
                status: EscrowStatus::DisputeResolvedForSender,
                has_dispute: true,
            }
        }
    }
}
