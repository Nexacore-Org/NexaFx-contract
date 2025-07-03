
use soroban_sdk::{contracttype, Address, Symbol, Vec, Map, Env};

/// Represents a swap offer in the contract's storage
#[contracttype]
#[derive(Clone)]
pub struct SwapOffer {
    /// The account that created this swap offer
    pub creator: Address,
    /// The token being offered
    pub offer_token: Address,
    /// The amount of tokens being offered
    pub offer_amount: i128,
    /// The token requested in exchange
    pub request_token: Address,
    /// The amount of tokens requested
    pub request_amount: i128,
    /// Timestamp when this offer expires
    pub expires_at: u64,
}

/// Represents the configuration of the swap contract
#[contracttype]
#[derive(Clone)]
pub struct SwapConfig {
    /// Admin address that can configure fees
    pub admin: Address,
    /// Fee percentage taken on each swap (basis points, e.g. 25 = 0.25%)
    pub fee_bps: u32,
    /// Address where fees are collected
    pub fee_collector: Address,
}

/// Events emitted by the swap contract
#[contracttype]
#[derive(Clone)]
pub enum SwapEvent {
    /// Emitted when a new swap offer is created
    OfferCreated {
        /// Unique ID of the offer
        offer_id: u64,
        /// Address of the creator
        creator: Address,
        /// Token being offered
        offer_token: Address,
        /// Amount being offered
        offer_amount: i128,
        /// Token being requested
        request_token: Address,
        /// Amount being requested
        request_amount: i128,
        /// Timestamp when this expires
        expires_at: u64,
    },
    /// Emitted when a swap offer is accepted
    OfferAccepted {
        /// Unique ID of the offer
        offer_id: u64,
        /// Address of the acceptor
        acceptor: Address,
    },
    /// Emitted when a swap offer is cancelled
    OfferCancelled {
        /// Unique ID of the offer
        offer_id: u64,
    },
    /// Emitted when fees are collected
    FeeCollected {
        /// Token collected as fee
        token: Address,
        /// Amount collected as fee
        amount: i128,
    },
}

/// Interface definition for swap contract functions
pub trait SwapTrait {
    /// Initializes the swap contract with default settings
    fn initialize(env: Env, admin: Address) -> SwapConfig;
    
    /// Updates the fee configuration
    fn update_fee(env: Env, fee_bps: u32, fee_collector: Address) -> SwapConfig;
    
    /// Creates a new swap offer
    fn create_offer(
        env: Env,
        offer_token: Address,
        offer_amount: i128,
        request_token: Address,
        request_amount: i128,
        expires_at: u64,
    ) -> u64;
    
    /// Accepts an existing swap offer
    fn accept_offer(env: Env, offer_id: u64) -> bool;
    
    /// Cancels an existing swap offer
    fn cancel_offer(env: Env, offer_id: u64) -> bool;
    
    /// Gets details of an existing swap offer
    fn get_offer(env: Env, offer_id: u64) -> SwapOffer;
    
    /// Gets the current contract configuration
    fn get_config(env: Env) -> SwapConfig;
} 