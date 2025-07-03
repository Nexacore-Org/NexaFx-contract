use crate::conversion::Currency;
use crate::errors::AppError;
use soroban_sdk::vec;
use soroban_sdk::xdr::FromXdr;
use soroban_sdk::{
    contracttype, log, token, xdr::ScAddressType, Address, Bytes, BytesN, Env, String,
};

/// Validates that an amount is positive
pub fn validate_positive_amount(amount: i128) -> Result<(), AppError> {
    if amount <= 0 {
        return Err(AppError::InvalidAmount);
    }
    Ok(())
}

/// Validates that a timestamp is in the future
pub fn validate_future_timestamp(env: &Env, timestamp: u64) -> Result<(), AppError> {
    if timestamp <= env.ledger().timestamp() {
        return Err(AppError::InvalidTimestamp);
    }
    Ok(())
}

/// Validates an address
pub fn validate_address(env: &Env, address: &Address) -> Result<(), AppError> {
    if address.to_string().is_empty() {
        return Err(AppError::InvalidAddress);
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
) -> Result<(), AppError> {
    if *amount <= 0 {
        return Err(AppError::InvalidAmount);
    }

    // Get balances before transfer
    let from_balance_before = get_token_balance(env, token_address, from);
    let to_balance_before = get_token_balance(env, token_address, to);

    let token_client = token::Client::new(env, token_address);
    token_client.transfer(from, to, amount);

    // Emit detailed transfer event with balance changes
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
pub fn get_token_balance(env: &Env, token_address: &Address, account: &Address) -> i128 {
    let token_client = token::Client::new(env, token_address);
    token_client.balance(account)
}

/// Computes exchange rate between two token amounts
pub fn compute_exchange_rate(offer_amount: i128, request_amount: i128) -> Result<i128, AppError> {
    if offer_amount <= 0 || request_amount <= 0 {
        return Err(AppError::InvalidAmount);
    }

    // Return rate scaled by 10^8 for precision
    let rate = (offer_amount * 100_000_000) / request_amount;
    Ok(rate)
}

/// Validates currency is supported
pub fn validate_currency_support(
    currency: &Currency,
    supported_currencies: &soroban_sdk::Vec<Currency>,
) -> Result<(), AppError> {
    for supported in supported_currencies.iter() {
        if supported == *currency {
            return Ok(());
        }
    }
    Err(AppError::UnsupportedCurrency)
}

/// Checks if user has sufficient balance for conversion
pub fn check_sufficient_balance(user_balance: i128, required_amount: i128) -> Result<(), AppError> {
    if user_balance < required_amount {
        return Err(AppError::InsufficientBalance);
    }
    Ok(())
}

/// Validates conversion limits
pub fn validate_conversion_limits(
    amount: i128,
    min_amount: i128,
    max_amount: i128,
) -> Result<(), AppError> {
    if amount < min_amount || amount > max_amount {
        return Err(AppError::ConversionLimitExceeded);
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
    env: &Env,
    duration: u64,
    max_duration: u64,
) -> Result<(), AppError> {
    if duration > max_duration {
        return Err(AppError::InvalidTimestamp);
    }
    Ok(())
}

/// Checks if exchange rate has expired
pub fn is_rate_expired(rate_updated_at: u64, validity_duration: u64, current_time: u64) -> bool {
    current_time > (rate_updated_at + validity_duration)
}

/// Atomic balance update helper
pub fn update_balance_atomically(
    env: &Env,
    user: &Address,
    currency: &Currency,
    current_balance: i128,
    amount_change: i128,
    is_debit: bool,
) -> Result<i128, AppError> {
    let new_balance = if is_debit {
        if current_balance < amount_change {
            return Err(AppError::InsufficientBalance);
        }
        current_balance - amount_change
    } else {
        current_balance + amount_change
    };
    log!(
        env,
        "Balance updated for {}: {} {} -> {}",
        user,
        get_currency_symbol(currency),
        current_balance,
        new_balance
    );
    Ok(new_balance)
}
pub fn validate_token_contract(env: &Env, _token_address: &Address) -> bool {
    true
}

pub fn validate_token_balance(env: &Env, _token_address: &Address, _amount: i128) -> bool {
    true
}

// derive wallet address from email

pub fn derive_wallet_address_from_email(env: &Env, email: &String) -> Result<Address, AppError> {
    if email.len() == 0 {
        return Err(AppError::InvalidAddress);
    }

    let len = email.len() as usize;
    if len > 256 {
        return Err(AppError::InvalidAddress);
    }

    let mut buf = [0u8; 256];
    email.copy_into_slice(&mut buf[..len]);
    let email_bytes = Bytes::from_slice(env, &buf[..len]);

    let hash: BytesN<32> = env.crypto().sha256(&email_bytes).into();

    let mut xdr: [u8; 40] = [0; 40];
    xdr[3] = 18;
    xdr[7] = 1;
    let slice: &mut [u8; 32] = (&mut xdr[8..40]).try_into().unwrap();
    hash.copy_into_slice(slice);

    let addr_bytes = Bytes::from_slice(env, &xdr);
    let address = Address::from_xdr(env, &addr_bytes).map_err(|_| AppError::InvalidAddress)?;

    Ok(address)
}
