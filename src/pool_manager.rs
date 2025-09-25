use soroban_sdk::{
    contract, contractimpl, contractmeta, contracttype, log, Address, Env, Vec,
};

use crate::conversion::Currency;
use crate::utils::{validate_address, validate_positive_amount};

/// Liquidity pool for a specific currency
#[contracttype]
#[derive(Clone)]
pub struct LiquidityPool {
    /// Currency of the pool
    pub currency: Currency,
    /// Total liquidity in the pool
    pub total_liquidity: i128,
    /// Available liquidity for conversions
    pub available_liquidity: i128,
    /// Reserved liquidity (locked in active conversions)
    pub reserved_liquidity: i128,
    /// Number of liquidity providers
    pub provider_count: u32,
    /// Pool creation timestamp
    pub created_at: u64,
    /// Last activity timestamp
    pub last_activity_at: u64,
    /// Minimum liquidity threshold
    pub min_liquidity_threshold: i128,
    /// Pool utilization rate (basis points)
    pub utilization_rate_bps: u32,
}

/// Individual liquidity provider position
#[contracttype]
#[derive(Clone)]
pub struct LiquidityPosition {
    /// Provider's address
    pub provider: Address,
    /// Currency of the position
    pub currency: Currency,
    /// Amount of liquidity provided
    pub liquidity_amount: i128,
    /// Share of the pool (basis points)
    pub pool_share_bps: u32,
    /// Timestamp when liquidity was added
    pub added_at: u64,
    /// Last time position was modified
    pub last_modified_at: u64,
    /// Accumulated rewards from conversions
    pub accumulated_rewards: i128,
    /// Lock period end timestamp (0 if not locked)
    pub lock_until: u64,
}

/// Pool manager configuration
#[contracttype]
#[derive(Clone)]
pub struct PoolManagerConfig {
    /// Administrator address
    pub admin: Address,
    /// Minimum liquidity amount per provider
    pub min_liquidity_amount: i128,
    /// Maximum liquidity amount per provider
    pub max_liquidity_amount: i128,
    /// Default liquidity lock period (seconds)
    pub default_lock_period: u64,
    /// Reward rate for liquidity providers (basis points)
    pub provider_reward_rate_bps: u32,
    /// Pool utilization threshold for warnings (basis points)
    pub utilization_warning_bps: u32,
    /// Emergency pause flag
    pub is_paused: bool,
}

/// Pool manager events
#[contracttype]
#[derive(Clone)]
pub enum PoolManagerEvent {
    /// Liquidity added to pool
    LiquidityAdded(Address, Currency, i128, u32),
    /// Liquidity removed from pool
    LiquidityRemoved(Address, Currency, i128, u32),
    /// Pool balance updated during conversion
    PoolBalanceUpdated(Currency, i128, i128, i128),
    /// Liquidity provider rewarded
    ProviderRewarded(Address, Currency, i128),
    /// Pool utilization warning
    PoolUtilizationWarning(Currency, u32),
    /// Emergency pause activated
    EmergencyPauseActivated(Address),
    /// Emergency pause deactivated
    EmergencyPauseDeactivated(Address),
}

/// Storage keys for pool manager
#[contracttype]
#[derive(Clone)]
pub enum PoolDataKey {
    /// Pool manager configuration
    PoolConfig,
    /// Liquidity pool for specific currency
    Pool(Currency),
    /// Liquidity position for provider and currency
    Position(Address, Currency),
    /// Total liquidity positions counter
    PositionCounter,
    /// Active pool currencies list
    ActiveCurrencies,
    /// Pool utilization history
    UtilizationHistory(Currency, u64), // Currency and day timestamp
    /// Provider rewards tracking
    ProviderRewards(Address),
    /// List of all providers for a specific currency
    CurrencyProviders(Currency),
}

#[contract]
pub struct PoolManagerContract;

// Contract metadata
contractmeta!(
    key = "Description",
    val = "Multi-currency liquidity pool manager for NexaFx conversion operations"
);

const DEFAULT_MIN_LIQUIDITY: i128 = 100_000_000; // 1 unit with 8 decimals
const DEFAULT_MAX_LIQUIDITY: i128 = 1_000_000_000_000; // 10,000 units with 8 decimals
const DEFAULT_LOCK_PERIOD: u64 = 86400; // 24 hours
const DEFAULT_REWARD_RATE_BPS: u32 = 10; // 0.1%
const DEFAULT_UTILIZATION_WARNING_BPS: u32 = 8000; // 80%
const MAX_UTILIZATION_BPS: u32 = 9500; // 95%
const BASIS_POINTS_DIVISOR: i128 = 10000;



#[contractimpl]
impl PoolManagerContract {
    /// Initialize the pool manager
    pub fn initialize_pool_manager(
        env: Env,
        admin: Address,
        min_liquidity: i128,
        max_liquidity: i128,
        lock_period: u64,
        reward_rate_bps: u32,
    ) -> PoolManagerConfig {
        admin.require_auth();
        validate_address(&env, &admin).unwrap();

        if min_liquidity <= 0 || max_liquidity <= min_liquidity {
            panic!("Invalid liquidity limits");
        }

        if reward_rate_bps > 1000 {
            panic!("Reward rate too high, maximum is 10%");
        }

        let config = PoolManagerConfig {
            admin: admin.clone(),
            min_liquidity_amount: min_liquidity,
            max_liquidity_amount: max_liquidity,
            default_lock_period: lock_period,
            provider_reward_rate_bps: reward_rate_bps,
            utilization_warning_bps: DEFAULT_UTILIZATION_WARNING_BPS,
            is_paused: false,
        };

        // Initialize active currencies list
        let active_currencies: Vec<Currency> = Vec::new(&env);
        env.storage().instance().set(&PoolDataKey::PoolConfig, &config);
        env.storage().instance().set(&PoolDataKey::ActiveCurrencies, &active_currencies);
        env.storage().instance().set(&PoolDataKey::PositionCounter, &0u64);

        log!(&env, "Pool manager initialized by admin: {}", admin);
        config
    }

    /// Add liquidity to a currency pool
    pub fn add_liquidity(
        env: Env,
        provider: Address,
        currency: Currency,
        amount: i128,
        lock_period: Option<u64>,
    ) -> LiquidityPosition {
        provider.require_auth();
        
        let config = Self::get_pool_config_internal(&env);
        if config.is_paused {
            panic!("Pool manager is paused");
        }

        validate_positive_amount(amount).unwrap();
        
        if amount < config.min_liquidity_amount || amount > config.max_liquidity_amount {
            panic!("Amount outside allowed liquidity limits");
        }

        let current_time = env.ledger().timestamp();
        let lock_until = lock_period.unwrap_or(config.default_lock_period) + current_time;

        // Get or create pool for currency
        let mut pool = Self::get_or_create_pool(&env, &currency);
        
        // Get or create provider position
        let mut position = Self::get_or_create_position(&env, &provider, &currency);

        // Update pool totals
        pool.total_liquidity += amount;
        pool.available_liquidity += amount;
        pool.last_activity_at = current_time;
        
        if position.liquidity_amount == 0 {
            pool.provider_count += 1;
        }

        // Update position
        position.liquidity_amount += amount;
        position.last_modified_at = current_time;
        position.lock_until = lock_until;

        // Update the providers list if this is a new provider
        if position.liquidity_amount == amount {
            // This is a new provider
            Self::add_provider_to_currency(&env, &provider, &currency);
        }

        // Store position first
        env.storage().instance().set(&PoolDataKey::Position(provider.clone(), currency.clone()), &position);
        
        // Recalculate shares for all providers in this currency pool
        Self::recalculate_all_shares(&env, &currency, pool.total_liquidity);

        // Update utilization rate
        pool.utilization_rate_bps = Self::calculate_utilization_rate(&pool);

        // Store updates
        env.storage().instance().set(&PoolDataKey::Pool(currency.clone()), &pool);

        // Update active currencies if this is a new pool
        Self::update_active_currencies(&env, &currency);

        // Get updated position to get correct share
        let updated_position = Self::get_position_internal(&env, &provider, &currency);

        // Emit event
        Self::publish_pool_event(
            &env,
            PoolManagerEvent::LiquidityAdded(
                provider.clone(),
                currency.clone(),
                amount,
                updated_position.pool_share_bps,
            ),
        );

        // Check utilization warning
        if pool.utilization_rate_bps > config.utilization_warning_bps {
            Self::publish_pool_event(
                &env,
                PoolManagerEvent::PoolUtilizationWarning(currency.clone(), pool.utilization_rate_bps),
            );
        }

        log!(
            &env,
            "Liquidity added: {} units by {}, share: {} bps",
            amount,
            provider,
            updated_position.pool_share_bps
        );

        updated_position
    }

    /// Remove liquidity from a currency pool
    pub fn remove_liquidity(
        env: Env,
        provider: Address,
        currency: Currency,
        amount: i128,
    ) -> LiquidityPosition {
        provider.require_auth();

        let config = Self::get_pool_config_internal(&env);
        if config.is_paused {
            panic!("Pool manager is paused");
        }

        validate_positive_amount(amount).unwrap();

        let current_time = env.ledger().timestamp();

        // Get provider position
        let mut position = Self::get_position_internal(&env, &provider, &currency);
        
        if position.lock_until > current_time {
            panic!("Liquidity is still locked");
        }

        if position.liquidity_amount < amount {
            panic!("Insufficient liquidity to remove");
        }

        // Get pool
        let mut pool = Self::get_pool_internal(&env, &currency);

        if pool.available_liquidity < amount {
            panic!("Pool has insufficient available liquidity");
        }

        // Update pool totals
        pool.total_liquidity -= amount;
        pool.available_liquidity -= amount;
        pool.last_activity_at = current_time;

        // Update position
        position.liquidity_amount -= amount;
        position.last_modified_at = current_time;

        if position.liquidity_amount == 0 {
            pool.provider_count -= 1;
        }

        // Handle provider removal if they have no liquidity left
        if position.liquidity_amount == 0 {
            Self::remove_provider_from_currency(&env, &provider, &currency);
        }

        // Store or remove position
        if position.liquidity_amount == 0 {
            // Remove position if no liquidity left
            env.storage().instance().remove(&PoolDataKey::Position(provider.clone(), currency.clone()));
        } else {
            // Store updated position
            env.storage().instance().set(&PoolDataKey::Position(provider.clone(), currency.clone()), &position);
        }
        
        // Recalculate shares for all providers in this currency pool
        Self::recalculate_all_shares(&env, &currency, pool.total_liquidity);

        // Update utilization rate
        pool.utilization_rate_bps = Self::calculate_utilization_rate(&pool);

        // Store updates
        env.storage().instance().set(&PoolDataKey::Pool(currency.clone()), &pool);

        // Get updated position for correct share (if still exists)
        let updated_position = if position.liquidity_amount > 0 {
            Self::get_position_internal(&env, &provider, &currency)
        } else {
            // For removed positions, set share to 0
            let mut removed_position = position.clone();
            removed_position.pool_share_bps = 0;
            removed_position
        };

        // Emit event
        Self::publish_pool_event(
            &env,
            PoolManagerEvent::LiquidityRemoved(
                provider.clone(),
                currency.clone(),
                amount,
                updated_position.pool_share_bps,
            ),
        );

        log!(
            &env,
            "Liquidity removed: {} units by {}, remaining share: {} bps",
            amount,
            provider,
            updated_position.pool_share_bps
        );

        updated_position
    }

    /// Update pool balance during conversion operations
    pub fn update_pool_on_conversion(
        env: Env,
        from_currency: Currency,
        to_currency: Currency,
        from_amount: i128,
        to_amount: i128,
    ) -> (LiquidityPool, LiquidityPool) {
        // This function should be called by the conversion contract
        // For now, we'll allow any caller but in production this should be restricted
        
        let current_time = env.ledger().timestamp();

        // Update source currency pool (liquidity consumed)
        let mut from_pool = Self::get_pool_internal(&env, &from_currency);
        if from_pool.available_liquidity < from_amount {
            panic!("Insufficient pool liquidity for conversion");
        }

        from_pool.available_liquidity -= from_amount;
        from_pool.reserved_liquidity += from_amount;
        from_pool.last_activity_at = current_time;
        from_pool.utilization_rate_bps = Self::calculate_utilization_rate(&from_pool);

        // Update target currency pool (liquidity added)
        let mut to_pool = Self::get_pool_internal(&env, &to_currency);
        to_pool.available_liquidity += to_amount;
        if to_pool.reserved_liquidity >= to_amount {
            to_pool.reserved_liquidity -= to_amount;
        }
        to_pool.last_activity_at = current_time;
        to_pool.utilization_rate_bps = Self::calculate_utilization_rate(&to_pool);

        // Store updates
        env.storage().instance().set(&PoolDataKey::Pool(from_currency.clone()), &from_pool);
        env.storage().instance().set(&PoolDataKey::Pool(to_currency.clone()), &to_pool);

        // Emit events
        Self::publish_pool_event(
            &env,
            PoolManagerEvent::PoolBalanceUpdated(
                from_currency.clone(),
                from_pool.total_liquidity,
                from_pool.available_liquidity,
                from_pool.reserved_liquidity,
            ),
        );

        Self::publish_pool_event(
            &env,
            PoolManagerEvent::PoolBalanceUpdated(
                to_currency.clone(),
                to_pool.total_liquidity,
                to_pool.available_liquidity,
                to_pool.reserved_liquidity,
            ),
        );

        log!(
            &env,
            "Pool balances updated for conversion: {} -> {} units",
            from_amount,
            to_amount
        );

        (from_pool, to_pool)
    }

    /// Distribute rewards to liquidity providers
    pub fn distribute_rewards(
        env: Env,
        currency: Currency,
        total_fee_amount: i128,
    ) -> Vec<(Address, i128)> {
        let config = Self::get_pool_config_internal(&env);
        config.admin.require_auth();

        let pool = Self::get_pool_internal(&env, &currency);
        let reward_amount = (total_fee_amount * i128::from(config.provider_reward_rate_bps)) / BASIS_POINTS_DIVISOR;

        if reward_amount <= 0 {
            return Vec::new(&env);
        }

        let rewards: Vec<(Address, i128)> = Vec::new(&env);
        let _active_currencies: Vec<Currency> = env.storage().instance().get(&PoolDataKey::ActiveCurrencies).unwrap_or_else(|| Vec::new(&env));

        // Find all positions for this currency
        // Note: In a real implementation, you'd want to maintain an index of positions per currency
        // For this example, we'll use a simplified approach

        log!(
            &env,
            "Distributing {} units in rewards to {} providers",
            reward_amount,
            pool.provider_count
        );

        rewards
    }

    /// Get liquidity pool information
    pub fn get_pool(env: Env, currency: Currency) -> LiquidityPool {
        Self::get_pool_internal(&env, &currency)
    }

    /// Get liquidity position for a provider
    pub fn get_position(env: Env, provider: Address, currency: Currency) -> LiquidityPosition {
        Self::get_position_internal(&env, &provider, &currency)
    }

    /// Get pool manager configuration
    pub fn get_pool_config(env: Env) -> PoolManagerConfig {
        Self::get_pool_config_internal(&env)
    }

    /// Get all active currencies with pools
    pub fn get_active_currencies(env: Env) -> Vec<Currency> {
        env.storage().instance().get(&PoolDataKey::ActiveCurrencies).unwrap_or_else(|| Vec::new(&env))
    }

    /// Emergency pause functionality
    pub fn emergency_pause(env: Env) -> bool {
        let mut config = Self::get_pool_config_internal(&env);
        config.admin.require_auth();

        config.is_paused = true;
        env.storage().instance().set(&PoolDataKey::PoolConfig, &config);

        Self::publish_pool_event(
            &env,
            PoolManagerEvent::EmergencyPauseActivated(config.admin.clone()),
        );

        log!(&env, "Emergency pause activated by admin: {}", config.admin);
        true
    }

    /// Resume operations after emergency pause
    pub fn resume_operations(env: Env) -> bool {
        let mut config = Self::get_pool_config_internal(&env);
        config.admin.require_auth();

        config.is_paused = false;
        env.storage().instance().set(&PoolDataKey::PoolConfig, &config);

        Self::publish_pool_event(
            &env,
            PoolManagerEvent::EmergencyPauseDeactivated(config.admin.clone()),
        );

        log!(&env, "Operations resumed by admin: {}", config.admin);
        true
    }

    // Private helper methods

    fn get_pool_config_internal(env: &Env) -> PoolManagerConfig {
        env.storage()
            .instance()
            .get(&PoolDataKey::PoolConfig)
            .unwrap_or_else(|| panic!("Pool manager not initialized"))
    }

    fn get_pool_internal(env: &Env, currency: &Currency) -> LiquidityPool {
        env.storage()
            .instance()
            .get(&PoolDataKey::Pool(currency.clone()))
            .unwrap_or_else(|| panic!("Pool not found for currency"))
    }

    fn get_or_create_pool(env: &Env, currency: &Currency) -> LiquidityPool {
        env.storage()
            .instance()
            .get(&PoolDataKey::Pool(currency.clone()))
            .unwrap_or_else(|| {
                let current_time = env.ledger().timestamp();
                LiquidityPool {
                    currency: currency.clone(),
                    total_liquidity: 0,
                    available_liquidity: 0,
                    reserved_liquidity: 0,
                    provider_count: 0,
                    created_at: current_time,
                    last_activity_at: current_time,
                    min_liquidity_threshold: DEFAULT_MIN_LIQUIDITY,
                    utilization_rate_bps: 0,
                }
            })
    }

    fn get_position_internal(env: &Env, provider: &Address, currency: &Currency) -> LiquidityPosition {
        env.storage()
            .instance()
            .get(&PoolDataKey::Position(provider.clone(), currency.clone()))
            .unwrap_or_else(|| panic!("Liquidity position not found"))
    }

    fn get_or_create_position(env: &Env, provider: &Address, currency: &Currency) -> LiquidityPosition {
        env.storage()
            .instance()
            .get(&PoolDataKey::Position(provider.clone(), currency.clone()))
            .unwrap_or_else(|| {
                let current_time = env.ledger().timestamp();
                LiquidityPosition {
                    provider: provider.clone(),
                    currency: currency.clone(),
                    liquidity_amount: 0,
                    pool_share_bps: 0,
                    added_at: current_time,
                    last_modified_at: current_time,
                    accumulated_rewards: 0,
                    lock_until: 0,
                }
            })
    }

    fn calculate_pool_share(position_amount: i128, total_pool_amount: i128) -> u32 {
        if total_pool_amount == 0 {
            return 0;
        }
        ((position_amount * BASIS_POINTS_DIVISOR) / total_pool_amount) as u32
    }

    fn calculate_utilization_rate(pool: &LiquidityPool) -> u32 {
        if pool.total_liquidity == 0 {
            return 0;
        }
        let utilized = pool.total_liquidity - pool.available_liquidity;
        ((utilized * BASIS_POINTS_DIVISOR) / pool.total_liquidity) as u32
    }

    fn update_active_currencies(env: &Env, currency: &Currency) {
        let mut active_currencies: Vec<Currency> = env
            .storage()
            .instance()
            .get(&PoolDataKey::ActiveCurrencies)
            .unwrap_or_else(|| Vec::new(env));

        // Check if currency already exists
        let mut found = false;
        for existing in active_currencies.iter() {
            if existing == *currency {
                found = true;
                break;
            }
        }

        if !found {
            active_currencies.push_back(currency.clone());
            env.storage().instance().set(&PoolDataKey::ActiveCurrencies, &active_currencies);
        }
    }

    fn publish_pool_event(env: &Env, event: PoolManagerEvent) {
        env.events().publish(("pool_manager",), event);
    }

    fn add_provider_to_currency(env: &Env, provider: &Address, currency: &Currency) {
        let mut providers: Vec<Address> = env
            .storage()
            .instance()
            .get(&PoolDataKey::CurrencyProviders(currency.clone()))
            .unwrap_or_else(|| Vec::new(env));

        // Check if provider already exists
        let mut found = false;
        for existing in providers.iter() {
            if existing == *provider {
                found = true;
                break;
            }
        }

        if !found {
            providers.push_back(provider.clone());
            env.storage()
                .instance()
                .set(&PoolDataKey::CurrencyProviders(currency.clone()), &providers);
        }
    }

    fn remove_provider_from_currency(env: &Env, provider: &Address, currency: &Currency) {
        let mut providers: Vec<Address> = env
            .storage()
            .instance()
            .get(&PoolDataKey::CurrencyProviders(currency.clone()))
            .unwrap_or_else(|| Vec::new(env));

        // Find and remove the provider
        let mut new_providers = Vec::new(env);
        for existing in providers.iter() {
            if existing != *provider {
                new_providers.push_back(existing);
            }
        }

        env.storage()
            .instance()
            .set(&PoolDataKey::CurrencyProviders(currency.clone()), &new_providers);
    }

    fn recalculate_all_shares(env: &Env, currency: &Currency, total_liquidity: i128) {
        let providers: Vec<Address> = env
            .storage()
            .instance()
            .get(&PoolDataKey::CurrencyProviders(currency.clone()))
            .unwrap_or_else(|| Vec::new(env));

        for provider in providers.iter() {
            if let Some(mut position) = env
                .storage()
                .instance()
                .get::<PoolDataKey, LiquidityPosition>(&PoolDataKey::Position(provider.clone(), currency.clone()))
            {
                position.pool_share_bps = if total_liquidity > 0 {
                    Self::calculate_pool_share(position.liquidity_amount, total_liquidity)
                } else {
                    0
                };
                
                env.storage()
                    .instance()
                    .set(&PoolDataKey::Position(provider.clone(), currency.clone()), &position);
            }
        }
    }
}
