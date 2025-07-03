use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, Env, Error
};

const MAX_BPS: u32 = 10000; // Represents 100%

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Config,
    TotalDistributed(Address), // Key for tracking total distributed amounts per token
}

const ERR_ALREADY_INITIALIZED: u32 = 1;
const ERR_INVALID_BPS_CONFIG: u32 = 2;
const ERR_NOT_INITIALIZED: u32 = 3;
const ERR_INVALID_BPS: u32 = 4;
const ERR_INVALID_FEE_AMOUNT: u32 = 5;
const ERR_FEE_DISTRIBUTION_FAILED: u32 = 7;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeeDistributionConfig {
    pub admin: Address,
    pub treasury_address: Address,
    pub reward_pool_address: Address,
    pub treasury_bps: u32,      // Basis points for treasury (e.g., 5000 for 50%)
    pub reward_pool_bps: u32,   // Basis points for reward pool (e.g., 5000 for 50%)
}

// To track total distributed amounts per token by this contract
#[contracttype]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TokenDistributionTotals {
    pub to_treasury: i128,
    pub to_reward_pool: i128,
}

#[contracttype]
pub struct FeeDistributedEvent {
    pub fee_token: Address,
    pub total_collected_fee: i128,
    pub treasury_dest: Address,
    pub treasury_amount: i128,
    pub reward_pool_dest: Address,
    pub reward_pool_amount: i128,
}

#[contract]
pub struct FeeSplitterContract;

#[contractimpl]
impl FeeSplitterContract {
    pub fn initialize_fees(
        env: Env,
        admin: Address,
        treasury_address: Address,
        reward_pool_address: Address,
        treasury_bps: u32,
        reward_pool_bps: u32,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Config) {
            return Err(Error::from_contract_error(ERR_ALREADY_INITIALIZED));
        }

        admin.require_auth();

        if treasury_bps > MAX_BPS || reward_pool_bps > MAX_BPS || (treasury_bps + reward_pool_bps) > MAX_BPS {
            return Err(Error::from_contract_error(ERR_INVALID_BPS_CONFIG));
        }

        let config = FeeDistributionConfig {
            admin,
            treasury_address,
            reward_pool_address,
            treasury_bps,
            reward_pool_bps,
        };
        env.storage().instance().set(&DataKey::Config, &config);
        Ok(())
    }

    pub fn update_fees_config(
        env: Env,
        treasury_address: Option<Address>,
        reward_pool_address: Option<Address>,
        treasury_bps: Option<u32>,
        reward_pool_bps: Option<u32>,
    ) -> Result<FeeDistributionConfig, Error> {
        let mut config: FeeDistributionConfig = env.storage().instance().get(&DataKey::Config)
            .ok_or_else(|| Error::from_contract_error(ERR_NOT_INITIALIZED))?;

        config.admin.require_auth();

        if let Some(addr) = treasury_address {
            config.treasury_address = addr;
        }
        if let Some(addr) = reward_pool_address {
            config.reward_pool_address = addr;
        }
        if let Some(bps) = treasury_bps {
            config.treasury_bps = bps;
        }
        if let Some(bps) = reward_pool_bps {
            config.reward_pool_bps = bps;
        }

        if config.treasury_bps > MAX_BPS || config.reward_pool_bps > MAX_BPS || (config.treasury_bps + config.reward_pool_bps) > MAX_BPS {
            return Err(Error::from_contract_error(ERR_INVALID_BPS));
        }

        env.storage().instance().set(&DataKey::Config, &config);
        Ok(config.clone())
    }

    pub fn get_fees_config(env: Env) -> Result<FeeDistributionConfig, Error> {
        env.storage().instance().get(&DataKey::Config)
            .ok_or_else(|| Error::from_contract_error(ERR_NOT_INITIALIZED))
    }

    /// Distributes collected fees to treasury and reward pools.
    /// This function should be called by the contract that collected the fees.
    /// `fee_collector_contract` is the address holding the `total_fee_amount`.
    pub fn distribute_fees(
        env: Env,
        fee_token: Address,
        total_fee_amount: i128,
        fee_collector_contract: Address, // The contract that holds the fees and calls this function
    ) -> Result<(), Error> {
        if total_fee_amount <= 0 {
            return Err(Error::from_contract_error(ERR_INVALID_FEE_AMOUNT));
        }

        // Authenticate the caller (must be the contract that collected the fees)
        // This ensures that only authorized contracts can trigger fee distribution from their balance.
        fee_collector_contract.require_auth();

        let config: FeeDistributionConfig = env.storage().instance().get(&DataKey::Config)
            .ok_or_else(|| Error::from_contract_error(ERR_NOT_INITIALIZED))?;

        let token_client = token::Client::new(&env, &fee_token);

        let treasury_amount = (total_fee_amount * i128::from(config.treasury_bps)) / i128::from(MAX_BPS);
        let reward_pool_amount = (total_fee_amount * i128::from(config.reward_pool_bps)) / i128::from(MAX_BPS);
        
        // Ensure the sum of distributed amounts does not exceed the total fee.
        // Any dust/remainder from bps calculation will remain with the fee_collector_contract.
        if treasury_amount + reward_pool_amount > total_fee_amount {
             return Err(Error::from_contract_error(ERR_FEE_DISTRIBUTION_FAILED));
        }

        if treasury_amount > 0 {
            token_client.transfer(&fee_collector_contract, &config.treasury_address, &treasury_amount);
        }

        if reward_pool_amount > 0 {
            token_client.transfer(&fee_collector_contract, &config.reward_pool_address, &reward_pool_amount);
        }
        
        // Emit event
        let fee_token_clone = fee_token.clone();
        env.events().publish(
            (symbol_short!("fee_distr"), fee_token_clone.clone()),
            FeeDistributedEvent {
                fee_token: fee_token_clone.clone(),
                total_collected_fee: total_fee_amount,
                treasury_dest: config.treasury_address.clone(),
                treasury_amount,
                reward_pool_dest: config.reward_pool_address.clone(),
                reward_pool_amount,
            },
        );

        // Update total distributed amounts if tracking within this contract
        let key = DataKey::TotalDistributed(fee_token_clone);
        let mut totals: TokenDistributionTotals = env.storage().instance().get(&key).unwrap_or_default();
        totals.to_treasury += treasury_amount;
        totals.to_reward_pool += reward_pool_amount;
        env.storage().instance().set(&key, &totals);

        Ok(())
    }

    // Function to get total distributed amounts for a token
    pub fn get_total_distributed(env: Env, token: Address) -> TokenDistributionTotals {
        env.storage().instance().get(&DataKey::TotalDistributed(token)).unwrap_or_default()
    }
}
