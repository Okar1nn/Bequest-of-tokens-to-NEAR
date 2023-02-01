use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub beneficiary: AccountId,
    pub donations: UnorderedMap<AccountId, u128>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            beneficiary: "@example.testnet".parse().unwrap(),
            donations: UnorderedMap::new(b"d"),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    pub fn init(beneficiary: AccountId) -> Self {
        Self {
            beneficiary,
            donations: UnorderedMap::new(b"d"),
        }
    }

    pub fn get_beneficiary(&self) -> AccountId {
        self.beneficiary.clone()
    }

    pub fn set_beneficiary(&mut self, beneficiary: AccountId) {
        self.beneficiary = beneficiary;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
}
