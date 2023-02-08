use crate::*;
use near_sdk::serde::Serialize;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Promise, Balance};
use near_sdk::json_types::U128;

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Donation {
    pub account_id: AccountId,
    pub total_amount: U128,
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn donate(&mut self) -> U128{
       let devisor: AccountId = env::predecessor_account_id();
       let devisor_amount: Balance = env::attached_deposit();

       let mut devisored_so_far = self.donations.get(&devisor).unwrap_or(0);

       let to_transfer: Balance = if devisored_so_far == 0{
        assert!(devisor_amount > STORAGE_COST, "Attach at least {} yoctoNEAR", STORAGE_COST);
        devisor_amount - STORAGE_COST
       }else{
        devisor_amount
       };
       devisored_so_far += devisor_amount;
       self.donations.insert(&devisor, &devisored_so_far);
       log!("Thank you {} for devisored {}! You devisored a total of {}", devisor.clone(), devisor_amount, devisored_so_far);
       Promise::new(self.beneficiary.clone()).transfer(to_transfer);

        U128(devisored_so_far)
    }

    pub fn get_devisor_for_account(&self, account_id:AccountId) -> Donation{
        Donation{
            account_id: account_id.clone(),
            total_amount: U128(self.donations.get(&account_id).unwrap_or(0))
        }
    }
    pub fn number_of_devisors(&self) -> u64{
        self.donations.len()
    }
    
    pub fn get_donations(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Donation>{
        let start = u128::from(from_index.unwrap_or(U128(0)));
        self.donations.keys().skip(start as usize).take(limit.unwrap_or(50) as usize).map(|account| self.get_devisor_for_account(account)).collect()
    }

}