use soroban_sdk::testutils::MockAuth;
use soroban_sdk::testutils::MockAuthInvoke;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env, IntoVal};
use stellar_multisig_contract::{mint::MintContract, mint::MintContractClient};

#[test]
fn test_successful_minting_by_admin() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let token = Address::generate(&env);

    // Dummy token contract (mocked)
    struct DummyToken;
    impl DummyToken {
        pub fn mint(env: &Env, to: &Address, amount: &i128) {
            env.events()
                .publish((symbol_short!("minted"), to.clone()), amount.clone());
        }
    }

    let mint_contract_id = env.register_contract(None, MintContract);
    env.mock_all_auths();

    let client = MintContractClient::new(&env, &mint_contract_id);
    client.init(&admin);
    client.mint_token(&user, &1000, &token);
}

#[test]
#[should_panic]
fn test_non_admin_cannot_mint() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let backend = Address::generate(&env);
    let attacker = Address::generate(&env);
    let token = Address::generate(&env);
    let user = Address::generate(&env);

    let contract_id = env.register_contract(None, MintContract);
    let client = MintContractClient::new(&env, &contract_id);

    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "mint_token",
            args: (&user, &1000i128, &token).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    client.init(&backend);
    client.mint_token(&attacker, &500, &token);
}
