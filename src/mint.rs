use crate::schema::TokenClient;
use soroban_sdk::symbol_short;
use soroban_sdk::{contract, contractclient, contractimpl, Address, Env};

#[contract]
pub struct MintContract;

#[contractimpl]
impl MintContract {
    pub fn init(env: Env, backend: Address) {
        backend.require_auth();
        // Store the admin (backend)
        env.storage()
            .persistent()
            .set(&symbol_short!("admin"), &backend);
    }

    // Only admin can mint
    pub fn mint_token(env: Env, recipient: Address, amount: i128, token: Address) {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&symbol_short!("admin"))
            .expect("admin not set");
        admin.require_auth();

        // Emit an event (for transparency)
        env.events()
            .publish((symbol_short!("mint"), recipient.clone()), amount);
    }
}
