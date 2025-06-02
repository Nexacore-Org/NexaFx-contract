#![no_std]
use soroban_sdk::{
    token, Address, Env, Error as ContractError, Symbol, log,
};

/// Validates that an amount is positive
pub fn validate_positive_amount(amount: i128) -> Result<(), ContractError> {
    if amount <= 0 {
        return Err(ContractError::from_contract_error("Amount must be positive"));
    }
    Ok(())
}

/// Validates that a timestamp is in the future
pub fn validate_future_timestamp(env: &Env, timestamp: u64) -> Result<(), ContractError> {
    if timestamp <= env.ledger().timestamp() {
        return Err(ContractError::from_contract_error("Timestamp must be in the future"));
    }
    Ok(())
}

/// Transfers tokens from one account to another
pub fn transfer_tokens(
    env: &Env,
    token_address: &Address,
    from: &Address,
    to: &Address,
    amount: &i128,
) -> Result<(), ContractError> {
    if *amount <= 0 {
        return Err(ContractError::from_contract_error("Amount must be positive"));
    }

    // Get balances before transfer
    let from_balance_before = get_token_balance(env, token_address, from);
    let to_balance_before = get_token_balance(env, token_address, to);
    
    let token_client = token::Client::new(env, token_address);
    token_client.transfer(from, to, amount);

    //Emit detailed transfer event with balance changes
    crate::event::EventEmitter::emit_token_transfer(
        env,
        token_address.clone(),
        from.clone(),
        to.clone(),
        *amount,
        from_balance_before - amount,
        to_balance_before + amount,
    );
    
    log!(env, "transferred {} tokens from {} to {}", amount, from, to);
    Ok(())
}

/// Gets the balance of an account for a specific token
pub fn get_token_balance(
    env: &Env,
    token_address: &Address,
    account: &Address,
) -> i128 {
    let token_client = token::Client::new(env, token_address);
    token_client.balance(account)
}

/// Computes exchange rate between two token amounts
pub fn compute_exchange_rate(
    offer_amount: i128,
    request_amount: i128,
) -> Result<f64, ContractError> {
    if offer_amount <= 0 || request_amount <= 0 {
        return Err(ContractError::from_contract_error("Amounts must be positive"));
    }
    
    // Convert to f64 for the division
    let rate = (offer_amount as f64) / (request_amount as f64);
    Ok(rate)
}

/// Validates a signature for a given payload
pub fn validate_address(env: &Env, address: &Address) -> Result<(), ContractError> {
    if !address.is_valid(env) {
        return Err(ContractError::from_contract_error("Invalid address"));
    }
    Ok(())
} 