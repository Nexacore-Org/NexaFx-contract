#![cfg(test)]

mod mock_token;

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _},
    Address, Env,
};
use stellar_multisig_contract::escrow::{EscrowClient, EscrowContract, EscrowStatus};
use mock_token::{MockToken, MockTokenClient};

fn setup_test_env() -> (Env, Address, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    
    // Deploy mock token
    let token_contract_id = env.register(MockToken, ());
    let token_client = MockTokenClient::new(&env, &token_contract_id);
    
    // Initialize token and mint some tokens to sender
    token_client.initialize(&1_000_000);
    token_client.mint(&sender, &10_000);
    
    // Deploy escrow contract
    let escrow_contract_id = env.register(EscrowContract, ());
    
    (env, escrow_contract_id, token_contract_id, sender, recipient)
}

#[test]
fn test_create_escrow_success() {
    let (env, escrow_contract_id, token_contract_id, sender, recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);
    let token_client = MockTokenClient::new(&env, &token_contract_id);

    // Create escrow
    let escrow_info = client.create(
        &sender,
        &recipient,
        &token_contract_id,
        &500,
        &3600, // 1 hour timeout
        &1800, // 30 minutes dispute period
    );

    assert_eq!(escrow_info.sender, sender);
    assert_eq!(escrow_info.recipient, recipient);
    assert_eq!(escrow_info.token, token_contract_id);
    assert_eq!(escrow_info.amount, 500);
    assert_eq!(escrow_info.status, EscrowStatus::Active);
    assert_eq!(escrow_info.dispute_period, 1800);
    assert!(!escrow_info.has_dispute);
    
    // Verify token was transferred from sender to escrow contract
    assert_eq!(token_client.balance(&sender), 10_000 - 500);
    assert_eq!(token_client.balance(&escrow_contract_id), 500);
}

#[test]
fn test_release_escrow_success() {
    let (env, escrow_contract_id, token_contract_id, sender, recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);
    let token_client = MockTokenClient::new(&env, &token_contract_id);

    // Create escrow
    let escrow_info = client.create(
        &sender,
        &recipient,
        &token_contract_id,
        &500,
        &3600,
        &1800,
    );

    // Release escrow
    let released_info = client.release(&escrow_info.id);

    assert_eq!(released_info.status, EscrowStatus::Released);
    
    // Verify tokens were transferred to recipient
    assert_eq!(token_client.balance(&recipient), 500);
    assert_eq!(token_client.balance(&escrow_contract_id), 0);
}

#[test]
fn test_initiate_dispute_by_sender() {
    let (env, escrow_contract_id, token_contract_id, sender, recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);

    // Create escrow
    let escrow_info = client.create(
        &sender,
        &recipient,
        &token_contract_id,
        &500,
        &3600,
        &1800,
    );

    // Initiate dispute by sender
    let disputed_info = client.initiate_dispute(&escrow_info.id, &symbol_short!("FRAUD"));

    assert_eq!(disputed_info.status, EscrowStatus::Disputed);
    assert!(disputed_info.has_dispute);

    let dispute = client.get_dispute_info(&escrow_info.id).unwrap();
    assert_eq!(dispute.initiated_by, sender);
    assert_eq!(dispute.reason, symbol_short!("FRAUD"));
    assert_eq!(dispute.dispute_period, 1800);
}

#[test]
#[should_panic(expected = "Dispute already initiated")]
fn test_duplicate_dispute_initiation() {
    let (env, escrow_contract_id, token_contract_id, sender, recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);

    // Create escrow
    let escrow_info = client.create(
        &sender,
        &recipient,
        &token_contract_id,
        &500,
        &3600,
        &1800,
    );

    // Initiate first dispute
    client.initiate_dispute(&escrow_info.id, &symbol_short!("FRAUD"));

    // Try to initiate second dispute - should panic
    client.initiate_dispute(&escrow_info.id, &symbol_short!("OTHER"));
}

#[test]
fn test_resolve_dispute_for_recipient() {
    let (env, escrow_contract_id, token_contract_id, sender, recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);
    let token_client = MockTokenClient::new(&env, &token_contract_id);

    // Create escrow and initiate dispute
    let escrow_info = client.create(
        &sender,
        &recipient,
        &token_contract_id,
        &500,
        &3600,
        &1800,
    );

    client.initiate_dispute(&escrow_info.id, &symbol_short!("FRAUD"));

    // Resolve dispute for recipient
    let resolved_info = client.resolve_dispute_for_recipient(&escrow_info.id);

    assert_eq!(
        resolved_info.status,
        EscrowStatus::DisputeResolvedForRecipient
    );
    
    // Verify tokens were transferred to recipient
    assert_eq!(token_client.balance(&recipient), 500);
}

#[test]
fn test_resolve_dispute_for_sender() {
    let (env, escrow_contract_id, token_contract_id, sender, recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);
    let token_client = MockTokenClient::new(&env, &token_contract_id);

    // Create escrow and initiate dispute
    let escrow_info = client.create(
        &sender,
        &recipient,
        &token_contract_id,
        &500,
        &3600,
        &1800,
    );

    client.initiate_dispute(&escrow_info.id, &symbol_short!("FRAUD"));

    // Resolve dispute for sender
    let resolved_info = client.resolve_dispute_for_sender(&escrow_info.id);

    assert_eq!(resolved_info.status, EscrowStatus::DisputeResolvedForSender);
    
    // Verify tokens were returned to sender
    assert_eq!(token_client.balance(&sender), 10_000); // Original balance restored
}

#[test]
fn test_can_dispute_functionality() {
    let (env, escrow_contract_id, token_contract_id, sender, recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);

    // Create escrow
    let escrow_info = client.create(
        &sender,
        &recipient,
        &token_contract_id,
        &500,
        &3600,
        &1800,
    );

    // Should be able to dispute initially
    assert!(client.can_dispute(&escrow_info.id));

    // Initiate dispute
    client.initiate_dispute(&escrow_info.id, &symbol_short!("FRAUD"));

    // Should not be able to dispute anymore
    assert!(!client.can_dispute(&escrow_info.id));
}

#[test]
#[should_panic(expected = "Escrow is not active or is disputed")]
fn test_release_disputed_escrow() {
    let (env, escrow_contract_id, token_contract_id, sender, recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);

    // Create escrow and initiate dispute
    let escrow_info = client.create(
        &sender,
        &recipient,
        &token_contract_id,
        &500,
        &3600,
        &1800,
    );

    client.initiate_dispute(&escrow_info.id, &symbol_short!("FRAUD"));

    // Try to release disputed escrow - should panic
    client.release(&escrow_info.id);
}

#[test]
#[should_panic(expected = "Sender and recipient cannot be the same")]
fn test_same_sender_recipient_validation() {
    let (env, escrow_contract_id, token_contract_id, sender, _recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);

    // Test same sender and recipient should fail
    client.create(
        &sender,
        &sender, // Same as sender
        &token_contract_id,
        &500,
        &3600,
        &1800,
    );
}

#[test]
#[should_panic(expected = "Amount must be positive")]
fn test_zero_amount_validation() {
    let (env, escrow_contract_id, token_contract_id, sender, recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);

    // Test zero amount should fail
    client.create(
        &sender,
        &recipient,
        &token_contract_id,
        &0, // Zero amount
        &3600,
        &1800,
    );
}

#[test]
#[should_panic(expected = "Timeout duration must be greater than dispute period")]
fn test_timeout_validation() {
    let (env, escrow_contract_id, token_contract_id, sender, recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);

    // Test timeout less than dispute period should fail
    client.create(
        &sender,
        &recipient,
        &token_contract_id,
        &500,
        &1000, // Timeout
        &2000, // Dispute period longer than timeout
    );
}

#[test]
fn test_admin_functions() {
    let (env, escrow_contract_id, _token_contract_id, _sender, _recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);
    let admin = Address::generate(&env);

    // Initialize contract
    client.initialize(&admin);

    // Test admin functions
    assert_eq!(client.get_admin(), admin);
    assert_eq!(client.get_dispute_fee(), 0);
    assert!(!client.is_paused());

    // Set dispute fee
    client.set_dispute_fee(&100);
    assert_eq!(client.get_dispute_fee(), 100);

    // Pause contract
    client.set_paused(&true);
    assert!(client.is_paused());

    // Unpause
    client.set_paused(&false);
    assert!(!client.is_paused());
}

#[test]
#[should_panic(expected = "Contract is paused")]
fn test_paused_contract_validation() {
    let (env, escrow_contract_id, token_contract_id, sender, recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);
    let admin = Address::generate(&env);

    // Initialize and pause contract
    client.initialize(&admin);
    client.set_paused(&true);

    // Try to create escrow while paused - should fail
    client.create(
        &sender,
        &recipient,
        &token_contract_id,
        &500,
        &3600,
        &1800,
    );
}

#[test]
fn test_query_functions() {
    let (env, escrow_contract_id, token_contract_id, sender, recipient) = setup_test_env();
    let client = EscrowClient::new(&env, &escrow_contract_id);

    // Initially no escrows
    assert_eq!(client.get_escrow_count(), 0);

    // Create an escrow
    let escrow_info = client.create(
        &sender,
        &recipient,
        &token_contract_id,
        &500,
        &3600,
        &1800,
    );

    // Check count increased
    assert_eq!(client.get_escrow_count(), 1);
    assert!(client.escrow_exists(&escrow_info.id));

    // Test filtering by status
    let active_escrows = client.get_escrows_by_status(&EscrowStatus::Active);
    assert_eq!(active_escrows.len(), 1);

    let disputed_escrows = client.get_escrows_by_status(&EscrowStatus::Disputed);
    assert_eq!(disputed_escrows.len(), 0);

    // Test filtering by participant
    let sender_escrows = client.get_escrows_by_participant(&sender);
    assert_eq!(sender_escrows.len(), 1);

    let recipient_escrows = client.get_escrows_by_participant(&recipient);
    assert_eq!(recipient_escrows.len(), 1);

    // Initiate dispute and check status filtering
    client.initiate_dispute(&escrow_info.id, &symbol_short!("FRAUD"));

    let active_escrows = client.get_escrows_by_status(&EscrowStatus::Active);
    assert_eq!(active_escrows.len(), 0);

    let disputed_escrows = client.get_escrows_by_status(&EscrowStatus::Disputed);
    assert_eq!(disputed_escrows.len(), 1);
}