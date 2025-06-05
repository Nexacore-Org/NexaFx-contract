#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env, 
    IntoVal, Symbol, Vec, Map, log, events
};

mod utils;
mod schema;
use schema::{SwapConfig, SwapEvent, SwapOffer, SwapTrait};

/// Contract state keys
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// Stores a mapping from offer_id to SwapOffer
    Offer(u64),
    /// Counter to generate unique offer IDs
    OfferCounter,
    /// Contract configuration
    Config,
}

#[contract]
pub struct SwapContract;

#[contractimpl]
impl SwapTrait for SwapContract {
    
    /// The initial contract configuration
    fn initialize(env: Env, admin: Address) -> SwapConfig {
        // Verify the contract is not already initialized
        if env.storage().has(&DataKey::Config) {
            panic!("Contract already initialized");
        }
        
        // Validate the admin address
        utils::validate_address(&env, &admin).unwrap();
        
        // Create initial config with 0.25% fee
        let config = SwapConfig {
            admin: admin.clone(),
            fee_bps: 25, // 0.25%
            fee_collector: admin,
        };
        
        // Store the configuration
        env.storage().set(&DataKey::Config, &config);
        env.storage().set(&DataKey::OfferCounter, &0u64);
        
        config
    }
   
    /// The updated contract configuration
    fn update_fee(env: Env, fee_bps: u32, fee_collector: Address) -> SwapConfig {
        // Get current config
        let mut config: SwapConfig = env.storage().get(&DataKey::Config).unwrap();
        
        // Only admin can update fees
        let caller = env.invoker();
        if caller != config.admin {
            panic!("Only admin can update fees");
        }
        
        // Max fee is 5%
        if fee_bps > 500 {
            panic!("Fee too high, maximum is 500 basis points (5%)");
        }
        
        // Validate fee collector address
        utils::validate_address(&env, &fee_collector).unwrap();
        
        // Update config
        config.fee_bps = fee_bps;
        config.fee_collector = fee_collector;
        
        // Save updated config
        env.storage().set(&DataKey::Config, &config);
        
        config
    }

    /// The offer ID for the created swap
    fn create_offer(
        env: Env,
        offer_token: Address,
        offer_amount: i128,
        request_token: Address,
        request_amount: i128,
        expires_at: u64,
    ) -> u64 {
        // Validate inputs
        utils::validate_positive_amount(offer_amount).unwrap();
        utils::validate_positive_amount(request_amount).unwrap();
        utils::validate_future_timestamp(&env, expires_at).unwrap();
        utils::validate_address(&env, &offer_token).unwrap();
        utils::validate_address(&env, &request_token).unwrap();
        
        // Get the creator of this offer
        let creator = env.invoker();
        
        // Transfer tokens from creator to the contract
        utils::transfer_tokens(
            &env,
            &offer_token,
            &creator,
            &env.current_contract_address(),
            &offer_amount
        ).unwrap();
        
        // Create the swap offer
        let offer = SwapOffer {
            creator: creator.clone(),
            offer_token,
            offer_amount,
            request_token,
            request_amount,
            expires_at,
        };
        
        // Generate a new offer ID
        let offer_counter: u64 = env.storage().get(&DataKey::OfferCounter).unwrap_or(0);
        let offer_id = offer_counter + 1;
        
        // Store the offer
        env.storage().set(&DataKey::Offer(offer_id), &offer);
        env.storage().set(&DataKey::OfferCounter, &offer_id);
        
        // Emit offer created event
        // ✨ NEW: Emit swap offer creation event
        crate::event::EventEmitter::emit_swap_offer_created(
            &env,
            offer_id,
            creator.clone(),
            offer_token.clone(),
            offer_amount,
            request_token.clone(),
            request_amount,
            expires_at,
        );

        offer_id

    }
    
    fn accept_offer(env: Env, offer_id: u64) -> bool {
        // Get the offer
        let offer: SwapOffer = env.storage().get(&DataKey::Offer(offer_id))
            .ok_or_else(|| panic!("Offer not found")).unwrap();
        
        // Check if the offer has expired
        if env.ledger().timestamp() > offer.expires_at {
            panic!("Offer has expired");
        }
        
        // Get the acceptor of this offer
        let acceptor = env.invoker();
        let contract_address = env.current_contract_address();
        
        // Get contract config for fee calculation
        let config: SwapConfig = env.storage().get(&DataKey::Config).unwrap();
        
        // Calculate fee on the offer amount (if any)
        let fee_amount = if config.fee_bps > 0 {
            offer.offer_amount * i128::from(config.fee_bps) / 10000
        } else {
            0
        };
        
        // Amount after fee
        let amount_after_fee = offer.offer_amount - fee_amount;
        
        // Transfer requested tokens from acceptor to offer creator
        utils::transfer_tokens(
            &env,
            &offer.request_token,
            &acceptor,
            &offer.creator,
            &offer.request_amount
        ).unwrap();
        
        // Transfer offered tokens from contract to acceptor
        utils::transfer_tokens(
            &env,
            &offer.offer_token,
            &contract_address,
            &acceptor,
            &amount_after_fee
        ).unwrap();
        
        // Transfer fee if applicable
        if fee_amount > 0 {
            utils::transfer_tokens(
                &env,
                &offer.offer_token,
                &contract_address,
                &config.fee_collector,
                &fee_amount
            ).unwrap();
            
            // Emit fee collected event
            events::emit(&env, SwapEvent::FeeCollected {
                token: offer.offer_token.clone(),
                amount: fee_amount,
            });
        }
        
        // Remove the offer
        env.storage().remove(&DataKey::Offer(offer_id));
        
         let event = crate::event::DeFiEvent::SwapOfferAccepted {
            topic: crate::event::SWAP_TOPIC,
            offer_id,
            creator: offer.creator.clone(),
            acceptor: acceptor.clone(),
            offer_token: offer.offer_token.clone(),
            offer_amount: amount_after_fee,
            request_token: offer.request_token.clone(),
            request_amount: offer.request_amount,
            fee_amount,
            fee_token: offer.offer_token.clone(),
            accepted_at: env.ledger().timestamp(),
            tx_hash: None,
        };
        crate::event::EventEmitter::emit_event(&env, event);
        }

        if fee_amount > 0 {
            let fee_event = crate::event::DeFiEvent::SwapFeeCollected {
                topic: crate::event::SWAP_TOPIC,
                offer_id,
                fee_collector: config.fee_collector.clone(),
                token: offer.offer_token.clone(),
                amount: fee_amount,
                fee_bps: config.fee_bps,
                collected_at: env.ledger().timestamp(),
                tx_hash: None,
            };
            crate::event::EventEmitter::emit_event(&env, fee_event);
        }

        true
    

    fn cancel_offer(env: Env, offer_id: u64) -> bool {
        // Get the offer
        let offer: SwapOffer = env.storage().get(&DataKey::Offer(offer_id))
            .ok_or_else(|| panic!("Offer not found")).unwrap();
        
        // Only the creator can cancel the offer
        let caller = env.invoker();
        if caller != offer.creator {
            panic!("Only the creator can cancel the offer");
        }
        
        let contract_address = env.current_contract_address();
        
        // Return offered tokens to the creator
        utils::transfer_tokens(
            &env,
            &offer.offer_token,
            &contract_address,
            &offer.creator,
            &offer.offer_amount
        ).unwrap();
        
        // Remove the offer
        env.storage().remove(&DataKey::Offer(offer_id));
        
        // Emit offer cancelled event
        events::emit(&env, SwapEvent::OfferCancelled {
            offer_id,
        });
        
        true
    }
    
    fn get_offer(env: Env, offer_id: u64) -> SwapOffer {
        env.storage().get(&DataKey::Offer(offer_id))
            .ok_or_else(|| panic!("Offer not found")).unwrap()
    }
    
    fn get_config(env: Env) -> SwapConfig {
        env.storage().get(&DataKey::Config)
            .ok_or_else(|| panic!("Contract not initialized")).unwrap()
    }
}
