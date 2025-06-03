use soroban_sdk::{contracttype, log, token, Address, Env};

use crate::conversion::Currency;

/// Custom error types for better error handling
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConversionError {
    InvalidAmount,
    InvalidAddress,
    InvalidTimestamp,
    InsufficientBalance,
    UnsupportedCurrency,
    RateExpired,
    ConversionLimitExceeded,
    Unauthorized,
}

/// Validates that an amount is positive
pub fn validate_positive_amount(amount: i128) -> Result<(), ConversionError> {
    if amount <= 0 {
        return Err(ConversionError::InvalidAmount);
    }
    Ok(())
}

/// Validates that a timestamp is in the future
pub fn validate_future_timestamp(env: &Env, timestamp: u64) -> Result<(), ConversionError> {
    if timestamp <= env.ledger().timestamp() {
        return Err(ConversionError::InvalidTimestamp);
    }
    Ok(())
}

/// Validates an address
pub fn validate_address(_env: &Env, address: &Address) -> Result<(), ConversionError> {
    if address.to_string().is_empty() {
        return Err(ConversionError::InvalidAddress);
    }
    Ok(())
}

/// Transfers tokens from one account to another
pub fn transfer_tokens(
    _env: &Env,
    token_address: &Address,
    from: &Address,
    to: &Address,
    amount: &i128,
) -> Result<(), ConversionError> {
    if *amount <= 0 {
        return Err(ConversionError::InvalidAmount);
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

    let token_client = token::Client::new(_env, token_address);
    token_client.transfer(from, to, amount);

    log!(
        _env,
        "Transferred {} tokens from {} to {}",
        amount,
        from,
        to
    );

    Ok(())
}

/// Gets the balance of an account for a specific token
pub fn get_token_balance(_env: &Env, token_address: &Address, account: &Address) -> i128 {
    let token_client = token::Client::new(_env, token_address);
    token_client.balance(account)
}

/// Computes exchange rate between two token amounts
pub fn compute_exchange_rate(
    offer_amount: i128,
    request_amount: i128,
) -> Result<i128, ConversionError> {
    if offer_amount <= 0 || request_amount <= 0 {
        return Err(ConversionError::InvalidAmount);
    }

    // Return rate scaled by 10^8 for precision
    let rate = (offer_amount * 100_000_000) / request_amount;
    Ok(rate)
}

/// Validates currency is supported
pub fn validate_currency_support(
    currency: &Currency,
    supported_currencies: &soroban_sdk::Vec<Currency>,
) -> Result<(), ConversionError> {
    for supported in supported_currencies.iter() {
        if supported == *currency {
            return Ok(());
        }
    }
    Err(ConversionError::UnsupportedCurrency)
}

/// Checks if user has sufficient balance for conversion
pub fn check_sufficient_balance(
    user_balance: i128,
    required_amount: i128,
) -> Result<(), ConversionError> {
    if user_balance < required_amount {
        return Err(ConversionError::InsufficientBalance);
    }
    Ok(())
}

/// Validates conversion limits
pub fn validate_conversion_limits(
    amount: i128,
    min_amount: i128,
    max_amount: i128,
) -> Result<(), ConversionError> {
    if amount < min_amount || amount > max_amount {
        return Err(ConversionError::ConversionLimitExceeded);
    }
    Ok(())
}

/// Calculates conversion amount with precision
pub fn calculate_conversion_amount(
    input_amount: i128,
    exchange_rate: i128,
    rate_precision: i128,
) -> i128 {
    (input_amount * exchange_rate) / rate_precision
}

/// Calculates platform fee
pub fn calculate_platform_fee(amount: i128, fee_basis_points: u32) -> i128 {
    (amount * i128::from(fee_basis_points)) / 10000
}

/// Formats currency display name
pub fn format_currency_name(currency: &Currency) -> &'static str {
    match currency {
        Currency::NGN => "Nigerian Naira",
        Currency::USD => "US Dollar",
        Currency::EUR => "Euro",
        Currency::GBP => "British Pound",
        Currency::BTC => "Bitcoin",
        Currency::ETH => "Ethereum",
    }
}

/// Gets currency symbol
pub fn get_currency_symbol(currency: &Currency) -> &'static str {
    match currency {
        Currency::NGN => "₦",
        Currency::USD => "$",
        Currency::EUR => "€",
        Currency::GBP => "£",
        Currency::BTC => "₿",
        Currency::ETH => "Ξ",
    }
}

/// Validates rate lock duration
pub fn validate_rate_lock_duration(
    _env: &Env,
    duration: u64,
    max_duration: u64,
) -> Result<(), ConversionError> {
    if duration > max_duration {
        return Err(ConversionError::InvalidTimestamp);
    }
    Ok(())
}

/// Checks if exchange rate has expired
pub fn is_rate_expired(rate_updated_at: u64, validity_duration: u64, current_time: u64) -> bool {
    current_time > (rate_updated_at + validity_duration)
}

/// Atomic balance update helper
pub fn update_balance_atomically(
    _env: &Env,
    user: &Address,
    currency: &Currency,
    current_balance: i128,
    amount_change: i128,
    is_debit: bool,
) -> Result<i128, ConversionError> {
    let new_balance = if is_debit {
        if current_balance < amount_change {
            return Err(ConversionError::InsufficientBalance);
        }
        current_balance - amount_change
    } else {
        current_balance + amount_change
    };

    log!(
        _env,
        "Balance updated for {}: {} {} -> {}",
        user,
        get_currency_symbol(currency),
        current_balance,
        new_balance
    );

    Ok(new_balance)
}

pub fn validate_token_contract(_env: &Env, _token_address: &Address) -> bool {
    true
}

pub fn validate_token_balance(_env: &Env, _token_address: &Address, _amount: i128) -> bool {
    true
}
