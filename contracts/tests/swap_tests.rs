use soroban_sdk::{
    testutils::{Address as _, Token},
    vec, Address, Env, IntoVal,
};

mod swap_contract {
    soroban_sdk::contractimport!(file = "../src/swap.rs");
}

#[test]
fn test_create_and_accept_swap() {
    let env = Env::default();
    let contract_id = env.register_contract(None, swap_contract::SwapContract {});
    let client = swap_contract::Client::new(&env, &contract_id);
    
    // Create admin
    let admin = Address::random(&env);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Create test tokens
    let naira_token_admin = Address::random(&env);
    let naira_token = env.register_stellar_asset_contract(naira_token_admin.clone());
    
    let usdc_token_admin = Address::random(&env);
    let usdc_token = env.register_stellar_asset_contract(usdc_token_admin.clone());
    
    // Create test users
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);
    
    // Mint tokens to users
    naira_token.with_source_account(&naira_token_admin).mint(&user1, &1000);
    usdc_token.with_source_account(&usdc_token_admin).mint(&user2, &100);
    
    // Check initial balances
    assert_eq!(naira_token.balance(&user1), 1000);
    assert_eq!(usdc_token.balance(&user2), 100);
    
    // Create a swap offer
    env.mock_all_auths();
    let expires_at = env.ledger().timestamp() + 1000;
    let offer_id = client.with_source_account(&user1).create_offer(
        &naira_token.address, &500, &usdc_token.address, &50, &expires_at
    );
    
    // Check balances after offer creation
    assert_eq!(naira_token.balance(&user1), 500);
    assert_eq!(naira_token.balance(&contract_id), 500);
    
    // Check offer details
    let offer = client.get_offer(&offer_id);
    assert_eq!(offer.creator, user1);
    assert_eq!(offer.offer_token, naira_token.address);
    assert_eq!(offer.offer_amount, 500);
    assert_eq!(offer.request_token, usdc_token.address);
    assert_eq!(offer.request_amount, 50);
    
    // Accept the offer
    client.with_source_account(&user2).accept_offer(&offer_id);
    
    // Calculate expected fee (0.25% of 500 = 1.25, rounded to 1)
    let expected_fee = 1;
    
    // Verify balances after swap
    assert_eq!(naira_token.balance(&user1), 500);
    assert_eq!(naira_token.balance(&user2), 499); // 500 - fee
    assert_eq!(naira_token.balance(&admin), expected_fee); // fee collected
    assert_eq!(usdc_token.balance(&user1), 50);
    assert_eq!(usdc_token.balance(&user2), 50);
}

#[test]
fn test_cancel_swap() {
    let env = Env::default();
    let contract_id = env.register_contract(None, swap_contract::SwapContract {});
    let client = swap_contract::Client::new(&env, &contract_id);
    
    // Create admin
    let admin = Address::random(&env);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Create test tokens
    let naira_token_admin = Address::random(&env);
    let naira_token = env.register_stellar_asset_contract(naira_token_admin.clone());
    
    // Create test user
    let user = Address::random(&env);
    
    // Mint tokens to user
    naira_token.with_source_account(&naira_token_admin).mint(&user, &1000);
    
    // Create a swap offer
    env.mock_all_auths();
    let expires_at = env.ledger().timestamp() + 1000;
    let offer_id = client.with_source_account(&user).create_offer(
        &naira_token.address, &500, &naira_token.address, &500, &expires_at
    );
    
    // Verify balance after creating offer
    assert_eq!(naira_token.balance(&user), 500);
    assert_eq!(naira_token.balance(&contract_id), 500);
    
    // Cancel the offer
    client.with_source_account(&user).cancel_offer(&offer_id);
    
    // Verify balance after cancellation
    assert_eq!(naira_token.balance(&user), 1000);
    assert_eq!(naira_token.balance(&contract_id), 0);
}

#[test]
#[should_panic(expected = "Offer has expired")]
fn test_expired_offer() {
    let env = Env::default();
    let contract_id = env.register_contract(None, swap_contract::SwapContract {});
    let client = swap_contract::Client::new(&env, &contract_id);
    
    // Create admin
    let admin = Address::random(&env);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Create test tokens
    let naira_token_admin = Address::random(&env);
    let naira_token = env.register_stellar_asset_contract(naira_token_admin.clone());
    
    let usdc_token_admin = Address::random(&env);
    let usdc_token = env.register_stellar_asset_contract(usdc_token_admin.clone());
    
    // Create test users
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);
    
    // Mint tokens to users
    naira_token.with_source_account(&naira_token_admin).mint(&user1, &1000);
    usdc_token.with_source_account(&usdc_token_admin).mint(&user2, &100);
    
    // Create a swap offer that expires immediately
    env.mock_all_auths();
    let expires_at = env.ledger().timestamp() + 10;
    let offer_id = client.with_source_account(&user1).create_offer(
        &naira_token.address, &500, &usdc_token.address, &50, &expires_at
    );
    
    // Advance time to make the offer expire
    env.ledger().set_timestamp(expires_at + 1);
    
    // Try to accept the expired offer, should panic
    client.with_source_account(&user2).accept_offer(&offer_id);
}

#[test]
fn test_fee_configuration() {
    let env = Env::default();
    let contract_id = env.register_contract(None, swap_contract::SwapContract {});
    let client = swap_contract::Client::new(&env, &contract_id);
    
    // Create admin
    let admin = Address::random(&env);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Get initial config
    let config = client.get_config();
    assert_eq!(config.admin, admin);
    assert_eq!(config.fee_bps, 25); // 0.25%
    
    // Update fee
    env.mock_all_auths();
    let new_fee_collector = Address::random(&env);
    client.with_source_account(&admin).update_fee(&50, &new_fee_collector);
    
    // Verify updated config
    let updated_config = client.get_config();
    assert_eq!(updated_config.admin, admin);
    assert_eq!(updated_config.fee_bps, 50); // 0.5%
    assert_eq!(updated_config.fee_collector, new_fee_collector);
    
    // Test fee calculation with new rate
    
    // Create test tokens
    let naira_token_admin = Address::random(&env);
    let naira_token = env.register_stellar_asset_contract(naira_token_admin.clone());
    
    let usdc_token_admin = Address::random(&env);
    let usdc_token = env.register_stellar_asset_contract(usdc_token_admin.clone());
    
    // Create test users
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);
    
    // Mint tokens to users
    naira_token.with_source_account(&naira_token_admin).mint(&user1, &1000);
    usdc_token.with_source_account(&usdc_token_admin).mint(&user2, &100);
    
    // Create a swap offer
    let expires_at = env.ledger().timestamp() + 1000;
    let offer_id = client.with_source_account(&user1).create_offer(
        &naira_token.address, &500, &usdc_token.address, &50, &expires_at
    );
    
    // Accept the offer
    client.with_source_account(&user2).accept_offer(&offer_id);
    
    // Calculate expected fee (0.5% of 500 = 2.5, rounded to 2)
    let expected_fee = 2;
    
    // Verify balances after swap
    assert_eq!(naira_token.balance(&user1), 500);
    assert_eq!(naira_token.balance(&user2), 498); // 500 - fee
    assert_eq!(naira_token.balance(&new_fee_collector), expected_fee); // fee collected
    assert_eq!(usdc_token.balance(&user1), 50);
    assert_eq!(usdc_token.balance(&user2), 50);
}

#[test]
fn test_naira_to_xlm_swap() {
    let env = Env::default();
    let contract_id = env.register_contract(None, swap_contract::SwapContract {});
    let client = swap_contract::Client::new(&env, &contract_id);
    
    // Create admin
    let admin = Address::random(&env);
    
    // Initialize the contract
    client.initialize(&admin);
    
    // Create test tokens
    let naira_token_admin = Address::random(&env);
    let naira_token = env.register_stellar_asset_contract(naira_token_admin.clone());
    
    let xlm_admin = Address::random(&env);
    let xlm_token = env.register_stellar_asset_contract(xlm_admin.clone());
    
    // Create test users
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);
    
    // Mint tokens to users
    naira_token.with_source_account(&naira_token_admin).mint(&user1, &100000);
    xlm_token.with_source_account(&xlm_admin).mint(&user2, &10);
    
    // Create a swap offer
    env.mock_all_auths();
    let expires_at = env.ledger().timestamp() + 1000;
    let offer_id = client.with_source_account(&user1).create_offer(
        &naira_token.address, &50000, &xlm_token.address, &5, &expires_at
    );
    
    // Accept the offer
    client.with_source_account(&user2).accept_offer(&offer_id);
    
    // Calculate expected fee (0.25% of 50000 = 125)
    let expected_fee = 125;
    
    // Verify balances after swap
    assert_eq!(naira_token.balance(&user1), 50000);
    assert_eq!(naira_token.balance(&user2), 49875); // 50000 - fee
    assert_eq!(naira_token.balance(&admin), expected_fee); // fee collected
    assert_eq!(xlm_token.balance(&user1), 5);
    assert_eq!(xlm_token.balance(&user2), 5);
} 