

use soroban_sdk::{contract, contractimpl, Env, Address, String, IntoVal};
use crate::utils::derive_wallet_address_from_email;
use crate::errors::AppError;


pub struct EmailToWalletContract;

impl EmailToWalletContract {
    pub fn get_wallet_from_email(env: Env, email: String) -> Result<Address, AppError> {
        derive_wallet_address_from_email(&env, &email)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

  #[test]
    fn same_email_same_address() {
        let env = Env::default();
        let email = String::from_slice(&env, "user@example.com");

        let addr1 = derive_wallet_address_from_email(&env, &email).expect("valid email");
        let addr2 = derive_wallet_address_from_email(&env, &email).expect("valid email");

        assert_eq!(addr1, addr2, "Same email should produce same address");
    }

    #[test]
    fn different_emails_different_addresses() {
        let env = Env::default();
        let email1 = String::from_slice(&env, "alice@example.com");
        let email2 = String::from_slice(&env, "bob@example.com");

        let addr1 = derive_wallet_address_from_email(&env, &email1).expect("valid email");
        let addr2 = derive_wallet_address_from_email(&env, &email2).expect("valid email");

        assert_ne!(addr1, addr2, "Different emails should produce different addresses");
    }

    #[test]
    fn empty_email_should_fail() {
        let env = Env::default();
        let empty_email = String::from_slice(&env, "");

        let result = derive_wallet_address_from_email(&env, &empty_email);
        assert!(matches!(result, Err(AppError::InvalidAddress)));
    }
}