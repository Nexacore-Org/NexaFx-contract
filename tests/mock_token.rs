use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Balance(Address),
    TotalSupply,
}

#[contract]
pub struct MockToken;

#[contractimpl]
impl MockToken {
    pub fn initialize(env: Env, total_supply: i128) {
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &total_supply);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let balance = Self::balance(env.clone(), to.clone());
        env.storage()
            .instance()
            .set(&DataKey::Balance(to), &(balance + amount));

        let total_supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(total_supply + amount));
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());
        let to_balance = Self::balance(env.clone(), to.clone());

        if from_balance < amount {
            panic!("insufficient balance");
        }

        env.storage()
            .instance()
            .set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage()
            .instance()
            .set(&DataKey::Balance(to), &(to_balance + amount));
    }

    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        _expiration_ledger: u32,
    ) {
        from.require_auth();
        // For simplicity, we'll just store the approval without expiration logic
        let key = (from, spender);
        env.storage().instance().set(&key, &amount);
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let key = (from, spender);
        env.storage().instance().get(&key).unwrap_or(0)
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if allowance < amount {
            panic!("insufficient allowance");
        }

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            panic!("insufficient balance");
        }

        let to_balance = Self::balance(env.clone(), to.clone());

        env.storage()
            .instance()
            .set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage()
            .instance()
            .set(&DataKey::Balance(to), &(to_balance + amount));

        let key = (from, spender);
        env.storage().instance().set(&key, &(allowance - amount));
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let balance = Self::balance(env.clone(), from.clone());
        if balance < amount {
            panic!("insufficient balance");
        }

        env.storage()
            .instance()
            .set(&DataKey::Balance(from), &(balance - amount));

        let total_supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(total_supply - amount));
    }

    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if allowance < amount {
            panic!("insufficient allowance");
        }

        let balance = Self::balance(env.clone(), from.clone());
        if balance < amount {
            panic!("insufficient balance");
        }

        env.storage()
            .instance()
            .set(&DataKey::Balance(from.clone()), &(balance - amount));

        let key = (from, spender);
        env.storage().instance().set(&key, &(allowance - amount));

        let total_supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(total_supply - amount));
    }

    pub fn decimals(_env: Env) -> u32 {
        7
    }

    pub fn name(env: Env) -> soroban_sdk::String {
        soroban_sdk::String::from_str(&env, "Mock Token")
    }

    pub fn symbol(env: Env) -> soroban_sdk::String {
        soroban_sdk::String::from_str(&env, "MOCK")
    }
}
