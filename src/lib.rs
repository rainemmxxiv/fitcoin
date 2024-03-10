// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};
use near_sdk::collections::LookupMap;

near_sdk::setup_alloc!();

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]

pub struct HealthContract {
    cotinine_levels: LookupMap<AccountId, f32>,
}

// Define the default, which automatically initializes the contract
impl Default for HealthContract {
    fn default() -> Self {
        Self {
            cotinine_levels: LookupMap::new(b"c"),
        }
    }
}

#[near_bindgen]
// Implement the contract structure
impl HealthContract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn update_cotinine_level(&mut self, user: AccountId, level: f32) {
        self.cotinine_levels.insert(&user, &level);
    }

    // Function to check cotinine level and reward user
    pub fn check_and_reward(&mut self, user: AccountId) {
        match self.cotinine_levels.get(&user) {
            //value of cotinine levels is in nanograms per milimetre 
            Some(level) if level < 1.0 => {
                // Transfers 50 NEAR to the user's account if cotinine level is below 1mg
                // 1 NEAR = 10^24 yoctoNEAR
                Promise::new(user).transfer(50 * 10u128.pow(24));
            },
            _ => env::log_str("The cotinine level is not below 1mg, or user not found."),
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello");
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy");
    }
}
