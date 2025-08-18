use soroban_sdk::symbol_short;
use soroban_sdk::{contractclient, contracttype, Address, Env, Map, Symbol, Vec};

#[contractclient(name = "TokenClient")]
pub trait TokenTrait {
    fn mint(env: Env, to: Address, amount: i128);
}

#[contracttype]
pub enum Event {
    FeeCollected(Address, i128),
    OfferCreated(u64, Address, i128),
    OfferAccepted(u64, Address),
    OfferCancelled(u64),
}

/// Represents a swap offer in the contract's storage
#[contracttype]
#[derive(Clone)]
pub struct SwapOffer {
    pub creator: Address,
    pub offer_token: Address,
    pub offer_amount: i128,
    pub request_token: Address,
    pub request_amount: i128,
    pub expires_at: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct SwapConfig {
    pub admin: Address,
    pub fee_bps: u32,
    pub fee_collector: Address,
}

pub trait SwapTrait {
    fn initialize(env: Env, admin: Address) -> SwapConfig;
    fn update_fee(env: Env, fee_bps: u32, fee_collector: Address) -> SwapConfig;
    fn create_offer(
        env: Env,
        offer_token: Address,
        offer_amount: i128,
        request_token: Address,
        request_amount: i128,
        expires_at: u64,
    ) -> u64;

    fn accept_offer(env: Env, offer_id: u64) -> bool;
    fn cancel_offer(env: Env, offer_id: u64) -> bool;
    fn get_offer(env: Env, offer_id: u64) -> SwapOffer;
    fn get_config(env: Env) -> SwapConfig;
}
