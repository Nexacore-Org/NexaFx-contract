use soroban_sdk::{contracttype};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppError {
    InvalidAmount,
    InvalidAddress,
    InvalidTimestamp,
    InsufficientBalance,
    UnsupportedCurrency,
    RateExpired,
    ConversionLimitExceeded,
    Unauthorized,
}
