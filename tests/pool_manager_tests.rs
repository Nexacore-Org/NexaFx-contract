#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger, LedgerInfo},
    Address, Env, InvokeError,
};
use stellar_multisig_contract::{
    conversion::Currency,
    pool_manager::{
        LiquidityPool, LiquidityPosition, PoolManagerConfig, PoolManagerContract, PoolManagerEvent,
    },
};

fn create_pool_manager_contract(env: &Env) -> Address {
    env.register_contract(None, PoolManagerContract)
}

fn advance_ledger(env: &Env, timestamp: u64) {
    env.ledger().set(LedgerInfo {
        timestamp,
        protocol_version: 22,
        sequence_number: 10,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 10,
        min_persistent_entry_ttl: 10,
        max_entry_ttl: 3110400,
    });
}

#[test]
fn test_initialize_pool_manager() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    let min_liquidity = 1_000_000_000; // 10 units
    let max_liquidity = 100_000_000_000; // 1000 units
    let lock_period = 86400; // 24 hours
    let reward_rate = 50; // 0.5%

    let config = client.initialize_pool_manager(
        &admin,
        &min_liquidity,
        &max_liquidity,
        &lock_period,
        &reward_rate,
    );

    assert_eq!(config.admin, admin);
    assert_eq!(config.min_liquidity_amount, min_liquidity);
    assert_eq!(config.max_liquidity_amount, max_liquidity);
    assert_eq!(config.default_lock_period, lock_period);
    assert_eq!(config.provider_reward_rate_bps, reward_rate);
    assert!(!config.is_paused);
}

#[test]
#[should_panic(expected = "Invalid liquidity limits")]
fn test_initialize_with_invalid_limits() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // max_liquidity <= min_liquidity should fail
    client.initialize_pool_manager(&admin, &1000, &500, &86400, &50);
}

#[test]
#[should_panic(expected = "Reward rate too high")]
fn test_initialize_with_high_reward_rate() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Reward rate > 10% should fail
    client.initialize_pool_manager(&admin, &1000, &10000, &86400, &1500);
}

#[test]
fn test_add_liquidity() {
    let env = Env::default();
    env.mock_all_auths();
    advance_ledger(&env, 1000);

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Initialize pool manager
    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);

    // Add liquidity
    let amount = 5_000_000_000; // 50 units
    let position = client.add_liquidity(&provider, &Currency::USD, &amount, &None);

    assert_eq!(position.provider, provider);
    assert_eq!(position.currency, Currency::USD);
    assert_eq!(position.liquidity_amount, amount);
    assert_eq!(position.pool_share_bps, 10000); // 100% of the pool
    assert!(position.lock_until > 1000);

    // Check pool state
    let pool = client.get_pool(&Currency::USD);
    assert_eq!(pool.total_liquidity, amount);
    assert_eq!(pool.available_liquidity, amount);
    assert_eq!(pool.reserved_liquidity, 0);
    assert_eq!(pool.provider_count, 1);
    assert_eq!(pool.utilization_rate_bps, 0);
}

#[test]
fn test_add_liquidity_multiple_providers() {
    let env = Env::default();
    env.mock_all_auths();
    advance_ledger(&env, 1000);

    let admin = Address::generate(&env);
    let provider1 = Address::generate(&env);
    let provider2 = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Initialize pool manager
    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);

    // First provider adds 60% of liquidity
    let amount1 = 6_000_000_000; // 60 units
    let position1 = client.add_liquidity(&provider1, &Currency::USD, &amount1, &None);

    // Second provider adds 40% of liquidity
    let amount2 = 4_000_000_000; // 40 units
    let position2 = client.add_liquidity(&provider2, &Currency::USD, &amount2, &None);

    // Check positions - need to retrieve current position states
    let current_position1 = client.get_position(&provider1, &Currency::USD);
    let current_position2 = client.get_position(&provider2, &Currency::USD);

    assert_eq!(current_position1.pool_share_bps, 6000); // 60%
    assert_eq!(current_position2.pool_share_bps, 4000); // 40%

    // Check pool state
    let pool = client.get_pool(&Currency::USD);
    assert_eq!(pool.total_liquidity, amount1 + amount2);
    assert_eq!(pool.provider_count, 2);
}

#[test]
#[should_panic(expected = "Amount outside allowed liquidity limits")]
fn test_add_liquidity_below_minimum() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);

    // Try to add liquidity below minimum
    client.add_liquidity(&provider, &Currency::USD, &500_000_000, &None);
}

#[test]
#[should_panic(expected = "Amount outside allowed liquidity limits")]
fn test_add_liquidity_above_maximum() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);

    // Try to add liquidity above maximum
    client.add_liquidity(&provider, &Currency::USD, &200_000_000_000, &None);
}

#[test]
fn test_remove_liquidity() {
    let env = Env::default();
    env.mock_all_auths();
    advance_ledger(&env, 1000);

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Initialize and add liquidity
    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);
    let amount = 5_000_000_000;
    client.add_liquidity(&provider, &Currency::USD, &amount, &Some(0)); // No lock period

    // Wait a bit to ensure we can remove liquidity
    advance_ledger(&env, 2000);

    // Remove half the liquidity
    let remove_amount = 2_500_000_000;
    let position = client.remove_liquidity(&provider, &Currency::USD, &remove_amount);

    assert_eq!(position.liquidity_amount, amount - remove_amount);
    assert_eq!(position.pool_share_bps, 10000); // Still 100% since only one provider

    // Check pool state
    let pool = client.get_pool(&Currency::USD);
    assert_eq!(pool.total_liquidity, amount - remove_amount);
    assert_eq!(pool.available_liquidity, amount - remove_amount);
    assert_eq!(pool.provider_count, 1);
}

#[test]
fn test_remove_all_liquidity() {
    let env = Env::default();
    env.mock_all_auths();
    advance_ledger(&env, 1000);

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Initialize and add liquidity
    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);
    let amount = 5_000_000_000;
    client.add_liquidity(&provider, &Currency::USD, &amount, &Some(0)); // No lock period

    advance_ledger(&env, 2000);

    // Remove all liquidity
    let position = client.remove_liquidity(&provider, &Currency::USD, &amount);

    assert_eq!(position.liquidity_amount, 0);
    assert_eq!(position.pool_share_bps, 0);

    // Check pool state
    let pool = client.get_pool(&Currency::USD);
    assert_eq!(pool.total_liquidity, 0);
    assert_eq!(pool.provider_count, 0);
}

#[test]
#[should_panic(expected = "Liquidity is still locked")]
fn test_remove_liquidity_while_locked() {
    let env = Env::default();
    env.mock_all_auths();
    advance_ledger(&env, 1000);

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Initialize and add liquidity with lock period
    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);
    let amount = 5_000_000_000;
    client.add_liquidity(&provider, &Currency::USD, &amount, &Some(86400)); // 24h lock

    // Try to remove immediately (should fail)
    client.remove_liquidity(&provider, &Currency::USD, &amount);
}

#[test]
#[should_panic(expected = "Insufficient liquidity to remove")]
fn test_remove_more_than_available() {
    let env = Env::default();
    env.mock_all_auths();
    advance_ledger(&env, 1000);

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Initialize and add liquidity
    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);
    let amount = 5_000_000_000;
    client.add_liquidity(&provider, &Currency::USD, &amount, &Some(0)); // No lock

    advance_ledger(&env, 2000);

    // Try to remove more than available
    client.remove_liquidity(&provider, &Currency::USD, &(amount + 1_000_000_000));
}

#[test]
fn test_update_pool_balance_on_conversion() {
    let env = Env::default();
    env.mock_all_auths();
    advance_ledger(&env, 1000);

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Initialize and add liquidity to both currencies
    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);

    let usd_amount = 10_000_000_000;
    let eur_amount = 8_000_000_000;

    client.add_liquidity(&provider, &Currency::USD, &usd_amount, &Some(0));
    client.add_liquidity(&provider, &Currency::EUR, &eur_amount, &Some(0));

    // Simulate conversion: 1000 USD -> 850 EUR
    let from_amount = 1_000_000_000;
    let to_amount = 850_000_000;

    let (from_pool, to_pool) =
        client.update_pool_on_conversion(&Currency::USD, &Currency::EUR, &from_amount, &to_amount);

    // Check USD pool (source)
    assert_eq!(from_pool.total_liquidity, usd_amount);
    assert_eq!(from_pool.available_liquidity, usd_amount - from_amount);
    assert_eq!(from_pool.reserved_liquidity, from_amount);
    assert!(from_pool.utilization_rate_bps > 0);

    // Check EUR pool (target)
    assert_eq!(to_pool.total_liquidity, eur_amount);
    assert_eq!(to_pool.available_liquidity, eur_amount + to_amount);
    assert_eq!(to_pool.reserved_liquidity, 0);
}

#[test]
#[should_panic(expected = "Insufficient pool liquidity for conversion")]
fn test_conversion_insufficient_liquidity() {
    let env = Env::default();
    env.mock_all_auths();
    advance_ledger(&env, 1000);

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Initialize with small liquidity
    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);
    let small_amount = 1_000_000_000; // 10 units
    client.add_liquidity(&provider, &Currency::USD, &small_amount, &Some(0));

    // Try to convert more than available
    let large_amount = 2_000_000_000; // 20 units
    client.update_pool_on_conversion(
        &Currency::USD,
        &Currency::EUR,
        &large_amount,
        &1_500_000_000,
    );
}

#[test]
fn test_emergency_pause_and_resume() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Initialize
    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);

    // Pause operations
    let paused = client.emergency_pause();
    assert!(paused);

    let config = client.get_pool_config();
    assert!(config.is_paused);

    // Resume operations
    let resumed = client.resume_operations();
    assert!(resumed);

    let config = client.get_pool_config();
    assert!(!config.is_paused);
}

#[test]
#[should_panic(expected = "Pool manager is paused")]
fn test_add_liquidity_while_paused() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Initialize and pause
    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);
    client.emergency_pause();

    // Try to add liquidity while paused (should fail)
    client.add_liquidity(&provider, &Currency::USD, &5_000_000_000, &None);
}

#[test]
fn test_get_active_currencies() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Initialize
    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);

    // Initially no active currencies
    let currencies = client.get_active_currencies();
    assert_eq!(currencies.len(), 0);

    // Add liquidity to USD pool
    client.add_liquidity(&provider, &Currency::USD, &5_000_000_000, &None);
    let currencies = client.get_active_currencies();
    assert_eq!(currencies.len(), 1);
    assert_eq!(currencies.get(0).unwrap(), Currency::USD);

    // Add liquidity to EUR pool
    client.add_liquidity(&provider, &Currency::EUR, &3_000_000_000, &None);
    let currencies = client.get_active_currencies();
    assert_eq!(currencies.len(), 2);
}

#[test]
fn test_multiple_currency_pools() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let provider1 = Address::generate(&env);
    let provider2 = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Initialize
    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);

    // Add liquidity to different currencies
    client.add_liquidity(&provider1, &Currency::USD, &10_000_000_000, &None);
    client.add_liquidity(&provider1, &Currency::EUR, &8_000_000_000, &None);
    client.add_liquidity(&provider2, &Currency::BTC, &5_000_000_000, &None);

    // Check each pool
    let usd_pool = client.get_pool(&Currency::USD);
    let eur_pool = client.get_pool(&Currency::EUR);
    let btc_pool = client.get_pool(&Currency::BTC);

    assert_eq!(usd_pool.total_liquidity, 10_000_000_000);
    assert_eq!(eur_pool.total_liquidity, 8_000_000_000);
    assert_eq!(btc_pool.total_liquidity, 5_000_000_000);

    // Check provider positions
    let usd_position = client.get_position(&provider1, &Currency::USD);
    let eur_position = client.get_position(&provider1, &Currency::EUR);
    let btc_position = client.get_position(&provider2, &Currency::BTC);

    assert_eq!(usd_position.pool_share_bps, 10000); // 100%
    assert_eq!(eur_position.pool_share_bps, 10000); // 100%
    assert_eq!(btc_position.pool_share_bps, 10000); // 100%
}

#[test]
fn test_pool_utilization_calculation() {
    let env = Env::default();
    env.mock_all_auths();
    advance_ledger(&env, 1000);

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_address = create_pool_manager_contract(&env);
    let client = PoolManagerContractClient::new(&env, &contract_address);

    // Initialize and add liquidity
    client.initialize_pool_manager(&admin, &1_000_000_000, &100_000_000_000, &86400, &50);
    let total_liquidity = 10_000_000_000; // 100 units
    client.add_liquidity(&provider, &Currency::USD, &total_liquidity, &Some(0));

    // Initial utilization should be 0%
    let pool = client.get_pool(&Currency::USD);
    assert_eq!(pool.utilization_rate_bps, 0);

    // Add liquidity to EUR pool first
    client.add_liquidity(&provider, &Currency::EUR, &total_liquidity, &Some(0));

    // Simulate 50% utilization through conversion
    let conversion_amount = 5_000_000_000; // 50 units
    client.update_pool_on_conversion(
        &Currency::USD,
        &Currency::EUR,
        &conversion_amount,
        &4_000_000_000, // 40 EUR units
    );

    // Check utilization is now 50%
    let pool = client.get_pool(&Currency::USD);
    assert_eq!(pool.utilization_rate_bps, 5000); // 50%
}

// Add this line at the end to ensure tests compile
use stellar_multisig_contract::pool_manager::PoolManagerContractClient;
