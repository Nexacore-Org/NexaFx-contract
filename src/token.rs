#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct TokenConfig {
    admin: Address,
    name: Symbol,
    symbol: Symbol,
    decimals: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Balance {
    amount: i128,
}

const CONFIG_KEY: Symbol = symbol_short!("CONFIG");

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        name: Symbol,
        symbol: Symbol,
        decimals: u32,
    ) -> TokenConfig {
        let config = TokenConfig {
            admin,
            name,
            symbol,
            decimals,
        };
        env.storage().instance().set(&CONFIG_KEY, &config);
        config
    }
    pub fn mint(env: Env, minter: Address, to: Address, amount: i128) {
        // Validate inputs
        if amount <= 0 {
            panic!("Amount must be positive");
        }

        // Require minter (admin) authorization
        minter.require_auth();

        // Check if minter is admin
        let config: TokenConfig = env.storage().instance().get(&CONFIG_KEY).unwrap();
        if minter != config.admin {
            panic!("Only admin can mint");
        }

        // Update balance
        let mut to_balance: Balance = env
            .storage()
            .instance()
            .get(&to)
            .unwrap_or(Balance { amount: 0 });
        to_balance.amount += amount;
        env.storage().instance().set(&to, &to_balance);

        // Emit token mint event
        let event = crate::event::DeFiEvent::TokenMinted(crate::event::TokenMintedData {
            token: env.current_contract_address(),
            to: to.clone(),
            amount,
            minter: minter.clone(),
            minted_at: env.ledger().timestamp(),
        });
        crate::event::EventEmitter::emit_event(&env, crate::event::TOKEN_TOPIC, event);
    }
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        // Validate inputs
        if amount <= 0 {
            panic!("Amount must be positive");
        }

        // Require from authorization
        from.require_auth();

        // Update balances
        let mut from_balance: Balance = env
            .storage()
            .instance()
            .get(&from)
            .unwrap_or(Balance { amount: 0 });
        let mut to_balance: Balance = env
            .storage()
            .instance()
            .get(&to)
            .unwrap_or(Balance { amount: 0 });

        if from_balance.amount < amount {
            panic!("Insufficient balance");
        }

        from_balance.amount -= amount;
        to_balance.amount += amount;

        env.storage().instance().set(&from, &from_balance);
        env.storage().instance().set(&to, &to_balance);

        // Emit token transfer event
        crate::event::EventEmitter::emit_token_transfer(
            &env,
            env.current_contract_address(),
            from.clone(),
            to.clone(),
            amount,
            from_balance.amount,
            to_balance.amount,
        );
    }
    pub fn balance(env: Env, of: Address) -> i128 {
        let balance: Balance = env
            .storage()
            .instance()
            .get(&of)
            .unwrap_or(Balance { amount: 0 });
        balance.amount
    }
    pub fn get_config(env: Env) -> TokenConfig {
        env.storage().instance().get(&CONFIG_KEY).unwrap()
    }
}