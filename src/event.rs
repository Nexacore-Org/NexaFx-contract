use soroban_sdk::{contracttype, Address, Symbol, Env, Vec, BytesN, Bytes};

// Event topics for efficient filtering and indexing
pub const ESCROW_TOPIC: Symbol = symbol_short!("ESCROW");
pub const SWAP_TOPIC: Symbol = symbol_short!("SWAP");
pub const MULTISIG_TOPIC: Symbol = symbol_short!("MULTISIG");
pub const TOKEN_TOPIC: Symbol = symbol_short!("TOKEN");
pub const SYSTEM_TOPIC: Symbol = symbol_short!("SYSTEM");

// Escrow event data structure
#[contracttype]
#[derive(Clone, Debug)]
pub struct EscrowCreatedData {
    pub escrow_id: Symbol,
    pub sender: Address,
    pub recipient: Address,
    pub token: Address,
    pub amount: i128,
    pub created_at: u64,
    pub timeout_at: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct EscrowReleasedData {
    pub escrow_id: Symbol,
    pub released_by: Address,
    pub recipient: Address,
    pub token: Address,
    pub amount: i128,
    pub released_at: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct EscrowRefundedData {
    pub escrow_id: Symbol,
    pub refunded_by: Address,
    pub sender: Address,
    pub token: Address,
    pub amount: i128,
    pub refunded_at: u64,
}

// Swap event data structures
#[contracttype]
#[derive(Clone, Debug)]
pub struct SwapOfferCreatedData {
    pub offer_id: u64,
    pub creator: Address,
    pub offer_token: Address,
    pub offer_amount: i128,
    pub request_token: Address,
    pub request_amount: i128,
    pub exchange_rate: i128,
    pub expires_at: u64,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct SwapOfferAcceptedData {
    pub offer_id: u64,
    pub creator: Address,
    pub acceptor: Address,
    pub offer_token: Address,
    pub offer_amount: i128,
    pub request_token: Address,
    pub request_amount: i128,
    pub fee_amount: i128,
    pub fee_token: Address,
    pub accepted_at: u64,
}

// Token event data structures
#[contracttype]
#[derive(Clone, Debug)]
pub struct TokenTransferredData {
    pub token: Address,
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub from_balance: i128,
    pub to_balance: i128,
    pub transferred_at: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct TokenMintedData {
    pub token: Address,
    pub to: Address,
    pub amount: i128,
    pub minter: Address,
    pub minted_at: u64,
}

// Multisig event data structures
#[contracttype]
#[derive(Clone, Debug)]
pub struct MultisigTransactionProposedData {
    pub nonce: u32,
    pub proposer: Address,
    pub operation_hash: BytesN<32>,
    pub threshold: u32,
    pub current_signatures: u32,
    pub proposed_at: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct MultisigTransactionExecutedData {
    pub nonce: u32,
    pub signers: Vec<Address>,
    pub operation_hash: BytesN<32>,
    pub executed_at: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct MultisigConfigUpdatedData {
    pub old_signers: Vec<Address>,
    pub new_signers: Vec<Address>,
    pub old_threshold: u32,
    pub new_threshold: u32,
    pub updated_at: u64,
}

// Wallet event data structures
#[contracttype]
#[derive(Clone, Debug)]
pub struct WalletToppedUpData {
    pub wallet: Address,
    pub token: Address,
    pub amount: i128,
    pub source: Address,
    pub new_balance: i128,
    pub topped_up_at: u64,
}

// System event data structures
#[contracttype]
#[derive(Clone, Debug)]
pub struct ContractErrorData {
    pub contract_address: Address,
    pub error_type: Symbol,
    pub error_message: Bytes,
    pub context_data: Bytes,
    pub occurred_at: u64,
}

// Comprehensive event system using tuple variants
#[contracttype]
#[derive(Clone, Debug)]
pub enum DeFiEvent {
    EscrowCreated(EscrowCreatedData),
    EscrowReleased(EscrowReleasedData),
    EscrowRefunded(EscrowRefundedData),
    SwapOfferCreated(SwapOfferCreatedData),
    SwapOfferAccepted(SwapOfferAcceptedData),
    TokenTransferred(TokenTransferredData),
    TokenMinted(TokenMintedData),
    MultisigTransactionProposed(MultisigTransactionProposedData),
    MultisigTransactionExecuted(MultisigTransactionExecutedData),
    MultisigConfigUpdated(MultisigConfigUpdatedData),
    WalletToppedUp(WalletToppedUpData),
    ContractError(ContractErrorData),
}

// Event emission utilities
pub struct EventEmitter;

impl EventEmitter {
    pub fn emit_event(env: &Env, topic: Symbol, event: DeFiEvent) {
        env.events().publish((topic,), event);
    }

    pub fn emit_escrow_created(
        env: &Env,
        escrow_id: Symbol,
        sender: Address,
        recipient: Address,
        token: Address,
        amount: i128,
        timeout_duration: u64,
    ) {
        let created_at = env.ledger().timestamp();
        let event_data = EscrowCreatedData {
            escrow_id,
            sender,
            recipient,
            token,
            amount,
            created_at,
            timeout_at: created_at + timeout_duration,
        };
        let event = DeFiEvent::EscrowCreated(event_data);
        Self::emit_event(env, ESCROW_TOPIC, event);
    }

    pub fn emit_escrow_released(
        env: &Env,
        escrow_id: Symbol,
        released_by: Address,
        recipient: Address,
        token: Address,
        amount: i128,
    ) {
        let event_data = EscrowReleasedData {
            escrow_id,
            released_by,
            recipient,
            token,
            amount,
            released_at: env.ledger().timestamp(),
        };
        let event = DeFiEvent::EscrowReleased(event_data);
        Self::emit_event(env, ESCROW_TOPIC, event);
    }

    pub fn emit_swap_offer_created(
        env: &Env,
        offer_id: u64,
        creator: Address,
        offer_token: Address,
        offer_amount: i128,
        request_token: Address,
        request_amount: i128,
        expires_at: u64,
    ) {
        let exchange_rate = if request_amount > 0 {
            (offer_amount * 1_000_000_000_000_000_000) / request_amount
        } else {
            0
        };

        let event_data = SwapOfferCreatedData {
            offer_id,
            creator,
            offer_token,
            offer_amount,
            request_token,
            request_amount,
            exchange_rate,
            expires_at,
            created_at: env.ledger().timestamp(),
        };
        let event = DeFiEvent::SwapOfferCreated(event_data);
        Self::emit_event(env, SWAP_TOPIC, event);
    }

    pub fn emit_token_transfer(
        env: &Env,
        token: Address,
        from: Address,
        to: Address,
        amount: i128,
        from_balance: i128,
        to_balance: i128,
    ) {
        let event_data = TokenTransferredData {
            token,
            from,
            to,
            amount,
            from_balance,
            to_balance,
            transferred_at: env.ledger().timestamp(),
        };
        let event = DeFiEvent::TokenTransferred(event_data);
        Self::emit_event(env, TOKEN_TOPIC, event);
    }

    pub fn emit_wallet_topped_up(
        env: &Env,
        wallet: Address,
        token: Address,
        amount: i128,
        source: Address,
        new_balance: i128,
    ) {
        let event_data = WalletToppedUpData {
            wallet,
            token,
            amount,
            source,
            new_balance,
            topped_up_at: env.ledger().timestamp(),
        };
        let event = DeFiEvent::WalletToppedUp(event_data);
        Self::emit_event(env, SYSTEM_TOPIC, event);
    }

    pub fn emit_contract_error(
        env: &Env,
        contract_address: Address,
        error_type: Symbol,
        error_message: &str,
        context_data: &[u8],
    ) {
        let event_data = ContractErrorData {
            contract_address,
            error_type,
            error_message: Bytes::from_slice(env, error_message.as_bytes()),
            context_data: Bytes::from_slice(env, context_data),
            occurred_at: env.ledger().timestamp(),
        };
        let event = DeFiEvent::ContractError(event_data);
        Self::emit_event(env, SYSTEM_TOPIC, event);
    }
}

// Event query utilities for backends and explorers
pub struct EventQuery;

impl EventQuery {
    pub fn escrow_events_filter() -> Symbol {
        ESCROW_TOPIC
    }

    pub fn swap_events_filter() -> Symbol {
        SWAP_TOPIC
    }

    pub fn multisig_events_filter() -> Symbol {
        MULTISIG_TOPIC
    }

    pub fn token_events_filter() -> Symbol {
        TOKEN_TOPIC
    }

    pub fn system_events_filter() -> Symbol {
        SYSTEM_TOPIC
    }
}

// Helper macro for importing symbol_short
use soroban_sdk::symbol_short;

