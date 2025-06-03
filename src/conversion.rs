#![no_std]
use soroban_sdk::{
    contract, contractimpl, contractmeta, contracttype, events, log, symbol_short, token, Address,
    Env, Map, String as SorobanString, Symbol, Vec,
};

use crate::utils::{
    get_token_balance, transfer_tokens, validate_address, validate_positive_amount,
};

use crate::events::publish;

/// Supported currencies for conversion
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Currency {
    NGN, // Nigerian Naira
    USD, // US Dollar
    EUR, // Euro
    GBP, // British Pound
    BTC, // Bitcoin
    ETH, // Ethereum
}

/// Exchange rate information
#[contracttype]
#[derive(Clone)]
pub struct ExchangeRate {
    /// Base currency
    pub from_currency: Currency,
    /// Target currency  
    pub to_currency: Currency,
    /// Exchange rate (scaled by 10^8 for precision)
    pub rate: i128,
    /// Timestamp when rate was set
    pub updated_at: u64,
    /// Rate validity duration in seconds
    pub validity_duration: u64,
    /// Whether the rate is locked for transactions
    pub is_locked: bool,
}

/// User balance information
#[contracttype]
#[derive(Clone)]
pub struct UserBalance {
    /// User's address
    pub user: Address,
    /// Currency balances map
    pub balances: Map<Currency, i128>,
    /// Last updated timestamp
    pub updated_at: u64,
}

/// Conversion transaction details
#[contracttype]
#[derive(Clone)]
pub struct ConversionTx {
    /// Transaction ID
    pub tx_id: Symbol,
    /// User performing conversion
    pub user: Address,
    /// Source currency
    pub from_currency: Currency,
    /// Target currency
    pub to_currency: Currency,
    /// Amount to convert (in source currency)
    pub amount: i128,
    /// Exchange rate used
    pub rate: i128,
    /// Amount received (after fees)
    pub amount_received: i128,
    /// Platform fee charged
    pub platform_fee: i128,
    /// Timestamp of conversion
    pub timestamp: u64,
    /// Transaction status
    pub status: ConversionStatus,
}

/// Status of conversion transaction
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConversionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

/// Platform configuration
#[contracttype]
#[derive(Clone)]
pub struct PlatformConfig {
    /// Platform admin
    pub admin: Address,
    /// Platform fee in basis points (e.g., 50 = 0.5%)
    pub fee_bps: u32,
    /// Fee collector address
    pub fee_collector: Address,
    /// Minimum conversion amount
    pub min_conversion_amount: i128,
    /// Maximum conversion amount per transaction
    pub max_conversion_amount: i128,
    /// Rate lock duration in seconds
    pub rate_lock_duration: u64,
}

/// Events emitted by the conversion contract
#[contracttype]
#[derive(Clone)]
pub enum ConversionEvent {
    /// Conversion completed successfully
    ConversionCompleted(Symbol, Address, Currency, Currency, i128, i128, i128),
    /// Exchange rate updated
    RateUpdated(Currency, Currency, i128, u64),
    /// Rate locked for transaction
    RateLocked(Currency, Currency, i128, u64),
    /// Fee collected
    FeeCollected(Currency, i128, Address),
}

/// Storage keys for the contract
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Platform configuration
    Config,
    /// Exchange rate for currency pair
    Rate(Currency, Currency),
    /// User balance
    Balance(Address),
    /// Conversion transaction
    Transaction(Symbol),
    /// Transaction counter
    TxCounter,
    /// Supported currencies list
    SupportedCurrencies,
}

#[contract]
pub struct ConversionContract;

// Contract metadata
contractmeta!(
    key = "Description",
    val = "Secure currency conversion contract with rate locking and fee management"
);

const RATE_PRECISION: i128 = 100_000_000; // 10^8 for rate precision
const MAX_FEE_BPS: u32 = 1000; // Maximum 10% fee

impl Currency {
    pub fn to_string(&self, env: &Env) -> SorobanString {
        match self {
            Currency::NGN => SorobanString::from_str(env, "NGN"),
            Currency::USD => SorobanString::from_str(env, "USD"),
            Currency::EUR => SorobanString::from_str(env, "EUR"),
            Currency::GBP => SorobanString::from_str(env, "GBP"),
            Currency::BTC => SorobanString::from_str(env, "BTC"),
            Currency::ETH => SorobanString::from_str(env, "ETH"),
        }
    }
}

#[contractimpl]
impl ConversionContract {
    /// Initialize the conversion contract
    pub fn initialize(
        env: Env,
        admin: Address,
        fee_bps: u32,
        fee_collector: Address,
        min_amount: i128,
        max_amount: i128,
    ) -> PlatformConfig {
        // Validate inputs
        admin.require_auth();
        validate_address(&env, &admin).unwrap();
        validate_address(&env, &fee_collector).unwrap();

        if fee_bps > MAX_FEE_BPS {
            panic!("Fee too high, maximum is {} basis points", MAX_FEE_BPS);
        }

        if min_amount <= 0 || max_amount <= min_amount {
            panic!("Invalid conversion amount limits");
        }

        let config = PlatformConfig {
            admin: admin.clone(),
            fee_bps,
            fee_collector,
            min_conversion_amount: min_amount,
            max_conversion_amount: max_amount,
            rate_lock_duration: 300, // 5 minutes default
        };

        // Initialize supported currencies
        let mut currencies = Vec::new(&env);
        currencies.push_back(Currency::NGN);
        currencies.push_back(Currency::USD);
        currencies.push_back(Currency::EUR);
        currencies.push_back(Currency::GBP);
        currencies.push_back(Currency::BTC);
        currencies.push_back(Currency::ETH);

        env.storage().instance().set(&DataKey::Config, &config);
        env.storage()
            .instance()
            .set(&DataKey::SupportedCurrencies, &currencies);
        env.storage().instance().set(&DataKey::TxCounter, &0u64);

        config
    }

    /// Update exchange rate for a currency pair
    pub fn update_rate(
        env: Env,
        from_currency: Currency,
        to_currency: Currency,
        rate: i128,
        validity_duration: u64,
    ) -> ExchangeRate {
        let config: PlatformConfig = env.storage().instance().get(&DataKey::Config).unwrap();

        // Only admin can update rates
        config.admin.require_auth();

        if rate <= 0 {
            panic!("Exchange rate must be positive");
        }

        let exchange_rate = ExchangeRate {
            from_currency: from_currency.clone(),
            to_currency: to_currency.clone(),
            rate,
            updated_at: env.ledger().timestamp(),
            validity_duration,
            is_locked: false,
        };

        env.storage().instance().set(
            &DataKey::Rate(from_currency.clone(), to_currency.clone()),
            &exchange_rate,
        );

        // Emit rate updated event
        publish(
            &env,
            ConversionEvent::RateUpdated(
                from_currency,
                to_currency,
                rate,
                exchange_rate.updated_at,
            ),
        );

        exchange_rate
    }

    /// Lock exchange rate for a transaction
    pub fn lock_rate(env: Env, from_currency: Currency, to_currency: Currency) -> ExchangeRate {
        let config: PlatformConfig = env.storage().instance().get(&DataKey::Config).unwrap();
        let mut rate_info: ExchangeRate = env
            .storage()
            .instance()
            .get(&DataKey::Rate(from_currency.clone(), to_currency.clone()))
            .unwrap_or_else(|| panic!("Exchange rate not found"));

        // Check if rate is still valid
        let current_time = env.ledger().timestamp();
        if current_time > rate_info.updated_at + rate_info.validity_duration {
            panic!("Exchange rate has expired");
        }

        // Lock the rate
        rate_info.is_locked = true;
        let locked_until = current_time + config.rate_lock_duration;

        env.storage().instance().set(
            &DataKey::Rate(from_currency.clone(), to_currency.clone()),
            &rate_info,
        );

        // Emit rate locked event
        publish(
            &env,
            ConversionEvent::RateLocked(from_currency, to_currency, rate_info.rate, locked_until),
        );

        rate_info
    }

    /// Perform currency conversion
    // pub fn convert_currency(
    //     env: Env,
    //     from_currency: Currency,
    //     to_currency: Currency,
    //     amount: i128,
    // ) -> ConversionTx {
    //     let user = env.current_contract_address();
    //     user.require_auth();

    pub fn convert_currency(
        env: Env,
        user: Address, // Add user parameter
        from_currency: Currency,
        to_currency: Currency,
        amount: i128,
    ) -> ConversionTx {
        user.require_auth();

        // Validate conversion parameters
        Self::validate_conversion(&env, &from_currency, &to_currency, amount);

        // Get and validate user balance
        let mut user_balance = Self::get_or_create_user_balance(&env, &user);
        let current_balance = user_balance
            .balances
            .get(from_currency.clone())
            .unwrap_or(0);

        if current_balance < amount {
            panic!("Insufficient balance for conversion");
        }

        // Get exchange rate
        let rate_info: ExchangeRate = env
            .storage()
            .instance()
            .get(&DataKey::Rate(from_currency.clone(), to_currency.clone()))
            .unwrap_or_else(|| panic!("Exchange rate not found"));

        // Validate rate is not expired
        let current_time = env.ledger().timestamp();
        if current_time > rate_info.updated_at + rate_info.validity_duration {
            panic!("Exchange rate has expired");
        }

        // Calculate conversion amounts
        let converted_amount = (amount * rate_info.rate) / RATE_PRECISION;
        let config: PlatformConfig = env.storage().instance().get(&DataKey::Config).unwrap();
        let platform_fee = Self::calculate_fee(converted_amount, config.fee_bps);
        let amount_received = converted_amount - platform_fee;

        // Generate transaction ID
        let tx_counter: u64 = env
            .storage()
            .instance()
            .get(&DataKey::TxCounter)
            .unwrap_or(0);
        let tx_id = Symbol::short(&format!("tx{}", tx_counter + 1));
        //         use soroban_sdk::symbol_short;
        // let tx_id = symbol_short!(&format!("tx{}", tx_counter + 1));

        // Update user balances atomically
        user_balance
            .balances
            .set(from_currency.clone(), current_balance - amount);
        let to_balance = user_balance.balances.get(to_currency.clone()).unwrap_or(0);
        user_balance
            .balances
            .set(to_currency.clone(), to_balance + amount_received);
        user_balance.updated_at = current_time;

        env.storage()
            .instance()
            .set(&DataKey::Balance(user.clone()), &user_balance);

        // Create conversion transaction record
        let conversion_tx = ConversionTx {
            tx_id: tx_id.clone(),
            user: user.clone(),
            from_currency: from_currency.clone(),
            to_currency: to_currency.clone(),
            amount,
            rate: rate_info.rate,
            amount_received,
            platform_fee,
            timestamp: current_time,
            status: ConversionStatus::Completed,
        };

        // Store transaction
        env.storage()
            .instance()
            .set(&DataKey::Transaction(tx_id.clone()), &conversion_tx);
        env.storage()
            .instance()
            .set(&DataKey::TxCounter, &(tx_counter + 1));

        // Collect platform fee
        if platform_fee > 0 {
            Self::collect_platform_fee(&env, &to_currency, platform_fee, &config.fee_collector);
        }

        // Emit conversion completed event
        publish(
            &env,
            ConversionEvent::ConversionCompleted(
                tx_id,
                user,
                from_currency,
                to_currency,
                amount,
                amount_received,
                platform_fee,
            ),
        );

        log!(
            &env,
            "Conversion completed: {} -> {}, amount: {}, received: {}",
            conversion_tx.from_currency.to_string(&env),
            conversion_tx.to_currency.to_string(&env),
            amount,
            amount_received
        );

        conversion_tx
    }

    /// Get user balance for all currencies
    pub fn get_user_balance(env: Env, user: Address) -> UserBalance {
        Self::get_or_create_user_balance(&env, &user)
    }

    /// Get conversion transaction details
    pub fn get_transaction(env: Env, tx_id: Symbol) -> ConversionTx {
        env.storage()
            .instance()
            .get(&DataKey::Transaction(tx_id))
            .unwrap_or_else(|| panic!("Transaction not found"))
    }

    /// Get current exchange rate
    pub fn get_rate(env: Env, from_currency: Currency, to_currency: Currency) -> ExchangeRate {
        env.storage()
            .instance()
            .get(&DataKey::Rate(from_currency, to_currency))
            .unwrap_or_else(|| panic!("Exchange rate not found"))
    }

    /// Get platform configuration
    pub fn get_config(env: Env) -> PlatformConfig {
        env.storage()
            .instance()
            .get(&DataKey::Config)
            .unwrap_or_else(|| panic!("Contract not initialized"))
    }

    // /// Deposit funds to user balance (for testing/admin purposes)
    // pub fn deposit(env: Env, user: Address, currency: Currency, amount: i128) {
    //     let config: PlatformConfig = env.storage().instance().get(&DataKey::Config).unwrap();
    //     config.admin.require_auth();

    //     validate_positive_amount(amount).unwrap();

    //     let mut user_balance = Self::get_or_create_user_balance(&env, &user);
    //     let current_balance = user_balance.balances.get(currency.clone()).unwrap_or(0);
    //     user_balance.balances.set(currency, current_balance + amount);
    //     user_balance.updated_at = env.ledger().timestamp();

    //     env.storage().instance().set(&DataKey::Balance(user), &user_balance);
    // }

    pub fn deposit(env: Env, user: Address, currency: Currency, amount: i128) {
        let config: PlatformConfig = env.storage().instance().get(&DataKey::Config).unwrap();

        // Either admin or the user themselves can deposit
        if env.current_contract_address() != config.admin {
            user.require_auth();
        } else {
            config.admin.require_auth();
        }

        validate_positive_amount(amount).unwrap();

        let mut user_balance = Self::get_or_create_user_balance(&env, &user);
        let current_balance = user_balance.balances.get(currency.clone()).unwrap_or(0);
        user_balance
            .balances
            .set(currency, current_balance + amount);
        user_balance.updated_at = env.ledger().timestamp();

        env.storage()
            .instance()
            .set(&DataKey::Balance(user), &user_balance);
    }

    // Private helper methods

    fn validate_conversion(
        env: &Env,
        from_currency: &Currency,
        to_currency: &Currency,
        amount: i128,
    ) {
        if from_currency == to_currency {
            panic!("Cannot convert to the same currency");
        }

        validate_positive_amount(amount).unwrap();

        let config: PlatformConfig = env.storage().instance().get(&DataKey::Config).unwrap();

        if amount < config.min_conversion_amount {
            panic!("Amount below minimum conversion limit");
        }

        if amount > config.max_conversion_amount {
            panic!("Amount exceeds maximum conversion limit");
        }

        // Validate currencies are supported
        let supported_currencies: Vec<Currency> = env
            .storage()
            .instance()
            .get(&DataKey::SupportedCurrencies)
            .unwrap();

        let mut from_supported = false;
        let mut to_supported = false;

        for currency in supported_currencies.iter() {
            if currency == *from_currency {
                from_supported = true;
            }
            if currency == *to_currency {
                to_supported = true;
            }
        }

        if !from_supported || !to_supported {
            panic!("Unsupported currency pair");
        }
    }

    fn get_or_create_user_balance(env: &Env, user: &Address) -> UserBalance {
        env.storage()
            .instance()
            .get(&DataKey::Balance(user.clone()))
            .unwrap_or_else(|| UserBalance {
                user: user.clone(),
                balances: Map::new(env),
                updated_at: env.ledger().timestamp(),
            })
    }

    fn calculate_fee(amount: i128, fee_bps: u32) -> i128 {
        (amount * i128::from(fee_bps)) / 10000
    }

    fn collect_platform_fee(
        env: &Env,
        currency: &Currency,
        fee_amount: i128,
        fee_collector: &Address,
    ) {
        publish(
            env,
            ConversionEvent::FeeCollected(currency.clone(), fee_amount, fee_collector.clone()),
        );

        log!(
            &env,
            "Fee collected: {} {} to {}",
            fee_amount,
            currency.to_string(env),
            fee_collector
        );
    }
}
