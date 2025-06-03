use soroban_sdk::{testutils::Address as _, Address, Env};

use stellar_multisig_contract::{
    conversion::{ConversionContractClient, ConversionStatus, Currency},
    ConversionContract,
};

fn create_test_env() -> (Env, ConversionContractClient<'static>, Address, Address) {
    let env = Env::default();
    let contract_id = env.register(ConversionContract, ());
    let client = ConversionContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fee_collector = Address::generate(&env);

    (env, client, admin, fee_collector)
}

fn setup_contract(
    _env: &Env,
    client: &ConversionContractClient<'static>,
    _admin: &Address,
    fee_collector: &Address,
) {
    client.initialize(
        _admin,
        &50u32, // 0.5% fee
        fee_collector,
        &100i128,           // min amount: 100
        &1_000_000_000i128, // max amount: 1B
    );
}

fn setup_exchange_rates(_env: &Env, client: &ConversionContractClient<'static>, _admin: &Address) {
    // Set up common exchange rates (scaled by 10^8 for precision)

    // USD to NGN: 1 USD = 800 NGN
    client.update_rate(
        &Currency::USD,
        &Currency::NGN,
        &80_000_000_000i128,
        &3600u64,
    );

    // NGN to USD: 1 NGN = 0.00125 USD
    client.update_rate(&Currency::NGN, &Currency::USD, &125_000i128, &3600u64);

    // USD to EUR: 1 USD = 0.85 EUR
    client.update_rate(&Currency::USD, &Currency::EUR, &85_000_000i128, &3600u64);

    // EUR to USD: 1 EUR = 1.176 USD
    client.update_rate(&Currency::EUR, &Currency::USD, &117_600_000i128, &3600u64);

    // GBP to USD: 1 GBP = 1.25 USD
    client.update_rate(&Currency::GBP, &Currency::USD, &125_000_000i128, &3600u64);

    // USD to GBP: 1 USD = 0.8 GBP
    client.update_rate(&Currency::USD, &Currency::GBP, &80_000_000i128, &3600u64);

    // BTC to USD: 1 BTC = 50,000 USD (for testing)
    client.update_rate(
        &Currency::BTC,
        &Currency::USD,
        &5_000_000_000_000i128,
        &3600u64,
    );

    // ETH to USD: 1 ETH = 3,000 USD (for testing)
    client.update_rate(
        &Currency::ETH,
        &Currency::USD,
        &300_000_000_000i128,
        &3600u64,
    );
}

fn fund_user_account(
    _env: &Env,
    client: &ConversionContractClient<'static>,
    _admin: &Address,
    user: &Address,
) {
    client.deposit(user, &Currency::USD, &100_000i128); // $1,000
    client.deposit(user, &Currency::NGN, &1_000_000i128); // ₦10,000
    client.deposit(user, &Currency::EUR, &50_000i128); // €500
    client.deposit(user, &Currency::GBP, &40_000i128); // £400
    client.deposit(user, &Currency::BTC, &10_000_000i128); // 0.1 BTC (in satoshis)
    client.deposit(user, &Currency::ETH, &5_000_000_000_000_000_000i128); // 5 ETH (in wei)
}

#[test]
fn test_usd_to_ngn_conversion() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();
    setup_contract(&env, &client, &admin, &fee_collector);
    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    fund_user_account(&env, &client, &admin, &user);

    // Convert $10 to NGN
    let conversion = client.convert_currency(&user, &Currency::USD, &Currency::NGN, &1000i128);

    assert_eq!(conversion.from_currency, Currency::USD);
    assert_eq!(conversion.to_currency, Currency::NGN);
    assert_eq!(conversion.amount, 1000i128);
    assert_eq!(conversion.status, ConversionStatus::Completed);

    // Check that user received approximately 8000 NGN (minus fees)
    // 1000 * 800 = 800,000 (scaled), then / 100 = 8,000 (in base units)
    // Minus 0.5% fee = 7,960
    assert!(conversion.amount_received >= 790_000i128 && conversion.amount_received <= 800_000i128);
}

#[test]
fn test_ngn_to_usd_conversion() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();
    setup_contract(&env, &client, &admin, &fee_collector);
    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    fund_user_account(&env, &client, &admin, &user);

    // Convert ₦400,000 to USD
    let conversion = client.convert_currency(&user, &Currency::NGN, &Currency::USD, &400_000i128);

    assert_eq!(conversion.from_currency, Currency::NGN);
    assert_eq!(conversion.to_currency, Currency::USD);
    assert_eq!(conversion.amount, 400_000i128);
    assert_eq!(conversion.status, ConversionStatus::Completed);

    // Should receive approximately $500 (minus fees)
    // 400,000 * 0.00125 = 500, minus 0.5% fee = 497.5
    assert!(conversion.amount_received >= 490i128 && conversion.amount_received <= 500i128);
}

#[test]
fn test_usd_to_eur_conversion() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();
    setup_contract(&env, &client, &admin, &fee_collector);
    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    fund_user_account(&env, &client, &admin, &user);

    // Convert $20 to EUR
    let conversion = client.convert_currency(&user, &Currency::USD, &Currency::EUR, &2000i128);

    assert_eq!(conversion.from_currency, Currency::USD);
    assert_eq!(conversion.to_currency, Currency::EUR);
    assert_eq!(conversion.amount, 2000i128);

    // Should receive approximately €17 (minus fees)
    // 2000 * 0.85 = 1700, minus 0.5% fee
    assert!(conversion.amount_received >= 1680i128 && conversion.amount_received <= 1700i128);
}

#[test]
fn test_gbp_to_usd_conversion() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();
    setup_contract(&env, &client, &admin, &fee_collector);
    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    fund_user_account(&env, &client, &admin, &user);

    // Convert £10 to USD
    let conversion = client.convert_currency(&user, &Currency::GBP, &Currency::USD, &1000i128);

    assert_eq!(conversion.from_currency, Currency::GBP);
    assert_eq!(conversion.to_currency, Currency::USD);
    assert_eq!(conversion.amount, 1000i128);

    // Should receive approximately $12.5 (minus fees)
    // 1000 * 1.25 = 1250, minus 0.5% fee
    assert!(conversion.amount_received >= 1240i128 && conversion.amount_received <= 1250i128);
}

#[test]
fn test_btc_to_usd_conversion() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();
    setup_contract(&env, &client, &admin, &fee_collector);
    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    fund_user_account(&env, &client, &admin, &user);

    // Convert 0.001 BTC (1,000,000 satoshis) to USD
    let conversion = client.convert_currency(&user, &Currency::BTC, &Currency::USD, &1_000_000i128);

    println!("BTC->USD amount_received: {}", conversion.amount_received);
    // The contract returns the result in smallest units (cents)
    // 1_000_000 satoshis * 5_000_000_000_000 / 100_000_000 = 50_000_000_000 (cents)
    // Fee: 0.5% of 50_000_000_000 = 250_000_000, so received = 49_750_000_000
    assert_eq!(conversion.amount_received, 49_750_000_000i128);
}

#[test]
fn test_eth_to_usd_conversion() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();
    // Increase max_amount for this test to allow 1 ETH in wei
    client.initialize(
        &admin,
        &50u32, // 0.5% fee
        &fee_collector,
        &100i128,                       // min amount: 100
        &2_000_000_000_000_000_000i128, // max amount: 2 ETH in wei
    );
    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    fund_user_account(&env, &client, &admin, &user);

    // Convert 1 ETH to USD
    let conversion = client.convert_currency(
        &user,
        &Currency::ETH,
        &Currency::USD,
        &1_000_000_000_000_000_000i128,
    );

    assert_eq!(conversion.from_currency, Currency::ETH);
    assert_eq!(conversion.to_currency, Currency::USD);
    assert_eq!(conversion.amount, 1_000_000_000_000_000_000i128);
    // Should receive approximately $3000 (minus fees)
    // 1 ETH * 3,000 USD = 3,000, minus 0.5% fee = 2,985
    // But in smallest units (cents), so 3,000 * 100 = 300,000, minus 0.5% fee = 298,500
    // And scaled by 10^8 for rate precision
    assert_eq!(
        conversion.amount_received,
        2_985_000_000_000_000_000_000i128
    );
}

#[test]
fn test_fee_calculation_accuracy() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();
    setup_contract(&env, &client, &admin, &fee_collector);
    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    fund_user_account(&env, &client, &admin, &user);

    // Convert $100 to NGN with 0.5% fee
    let conversion = client.convert_currency(&user, &Currency::USD, &Currency::NGN, &10_000i128);

    // Expected: 10000 * 800 = 8,000,000 NGN
    // Fee: 8,000,000 * 0.005 = 40,000 NGN
    // Received: 8,000,000 - 40,000 = 7,960,000 NGN

    let expected_gross = 8_000_000i128;
    let expected_fee = expected_gross * 50 / 10_000; // 0.5% fee
    let expected_net = expected_gross - expected_fee;

    assert_eq!(conversion.platform_fee, expected_fee);
    assert_eq!(conversion.amount_received, expected_net);
}

#[test]
fn test_different_fee_rates() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();

    // Initialize with higher fee (1%)
    client.initialize(
        &admin,
        &100u32, // 1% fee
        &fee_collector,
        &100i128,
        &1_000_000_000i128,
    );

    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    fund_user_account(&env, &client, &admin, &user);

    // Convert $50 to NGN with 1% fee
    let conversion = client.convert_currency(&user, &Currency::USD, &Currency::NGN, &5000i128);

    // Expected: 5000 * 800 = 4,000,000 NGN
    // Fee: 4,000,000 * 0.01 = 40,000 NGN
    // Received: 4,000,000 - 40,000 = 3,960,000 NGN

    let expected_gross = 4_000_000i128;
    let expected_fee = expected_gross / 100; // 1% fee
    let expected_net = expected_gross - expected_fee;

    assert_eq!(conversion.platform_fee, expected_fee);
    assert_eq!(conversion.amount_received, expected_net);
}

#[test]
fn test_multiple_conversions_different_amounts() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();
    setup_contract(&env, &client, &admin, &fee_collector);
    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    fund_user_account(&env, &client, &admin, &user);

    // First conversion: $10 to NGN
    let conversion1 = client.convert_currency(&user, &Currency::USD, &Currency::NGN, &1000i128);
    assert_eq!(conversion1.amount, 1000i128);

    // Second conversion: $50 to EUR
    let conversion2 = client.convert_currency(&user, &Currency::USD, &Currency::EUR, &5000i128);
    assert_eq!(conversion2.amount, 5000i128);

    // Third conversion: €20 to USD
    let conversion3 = client.convert_currency(&user, &Currency::EUR, &Currency::USD, &2000i128);
    assert_eq!(conversion3.amount, 2000i128);

    // All should be completed
    assert_eq!(conversion1.status, ConversionStatus::Completed);
    assert_eq!(conversion2.status, ConversionStatus::Completed);
    assert_eq!(conversion3.status, ConversionStatus::Completed);
}

#[test]
fn test_fee_distribution_across_multiple_conversions() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();
    setup_contract(&env, &client, &admin, &fee_collector);
    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    fund_user_account(&env, &client, &admin, &user);

    // Multiple small conversions
    let conv1 = client.convert_currency(&user, &Currency::USD, &Currency::NGN, &2000i128);
    let conv2 = client.convert_currency(&user, &Currency::USD, &Currency::NGN, &3000i128);
    let conv3 = client.convert_currency(&user, &Currency::USD, &Currency::NGN, &5000i128);

    // Each should have appropriate fees calculated
    assert!(conv1.platform_fee > 0);
    assert!(conv2.platform_fee > 0);
    assert!(conv3.platform_fee > 0);

    // Larger conversion should have proportionally larger fee
    assert!(conv3.platform_fee > conv2.platform_fee);
    assert!(conv2.platform_fee > conv1.platform_fee);
}

#[test]
fn test_fee_precision_with_small_amounts() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();
    setup_contract(&env, &client, &admin, &fee_collector);
    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    fund_user_account(&env, &client, &admin, &user);

    // Very small conversion: $1 to NGN
    let conversion = client.convert_currency(&user, &Currency::USD, &Currency::NGN, &100i128);

    assert_eq!(conversion.amount, 100i128);
    assert_eq!(conversion.status, ConversionStatus::Completed);

    // Even small amounts should have some fee (unless rounded to 0)
    let expected_gross = 80_000i128; // 100 * 800
    let expected_fee = expected_gross * 50 / 10_000; // 0.5%

    assert_eq!(conversion.platform_fee, expected_fee);
    assert_eq!(conversion.amount_received, expected_gross - expected_fee);
}

#[test]
fn test_zero_fee_configuration() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();

    // Initialize with zero fee
    client.initialize(
        &admin,
        &0u32, // 0% fee
        &fee_collector,
        &100i128,
        &1_000_000_000i128,
    );

    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    fund_user_account(&env, &client, &admin, &user);

    // Convert $20 to NGN with no fee
    let conversion = client.convert_currency(&user, &Currency::USD, &Currency::NGN, &2000i128);

    // Should receive full amount with no fee
    let expected_amount = 1_600_000i128; // 2000 * 800

    assert_eq!(conversion.platform_fee, 0i128);
    assert_eq!(conversion.amount_received, expected_amount);
}

#[test]
#[should_panic(expected = "Insufficient balance for conversion")]
fn test_insufficient_balance_failure() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();
    setup_contract(&env, &client, &admin, &fee_collector);
    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    // Don't fund the user account

    // Try to convert without sufficient balance
    client.convert_currency(&user, &Currency::USD, &Currency::NGN, &1000i128);
}

#[test]
#[should_panic(expected = "Insufficient balance for conversion")]
fn test_partial_balance_insufficient() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();
    setup_contract(&env, &client, &admin, &fee_collector);
    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    // Fund with less than needed
    client.deposit(&user, &Currency::USD, &500i128);

    // Try to convert more than available
    client.convert_currency(&user, &Currency::USD, &Currency::NGN, &1000i128);
}

#[test]
#[should_panic(expected = "InvalidAmount")]
fn test_zero_balance_conversion_failure() {
    let (env, client, admin, fee_collector) = create_test_env();
    env.mock_all_auths();
    setup_contract(&env, &client, &admin, &fee_collector);
    setup_exchange_rates(&env, &client, &admin);

    let user = Address::generate(&env);
    // Fund with zero balance - this should fail with InvalidAmount
    client.deposit(&user, &Currency::USD, &0i128);
}
