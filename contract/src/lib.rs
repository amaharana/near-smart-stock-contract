// TODO turn on halt on warning

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::collections::LookupMap;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    total_shares: u32,
    shares_outstanding: u32,
    share_ownership: LookupMap<String, u32>
}

impl Default for Contract {
  fn default() -> Self {
    Self {
      // TODO replace defaults with init parameters

      // total shares issued by the company
      total_shares: 1_000_000_000,

      // shares outstanding in market, available to buy
      shares_outstanding: 1_000_000_000,

      // TODO: what does b"a" signify here?
      // who has bought how many shares?
      share_ownership: LookupMap::new(b"a"),
    }
  }
}

#[near_bindgen]
impl Contract {
    pub fn buy_shares(&mut self, new_shares: u32) {
        let account_id = env::signer_account_id();

        if new_shares > self.shares_outstanding {
            panic!("Cannot buy more shares ({}) than available ({}).", new_shares, self.shares_outstanding);
        }

        let current_shares = self.share_ownership.get(&account_id).unwrap_or_default();
        let new_total = current_shares + new_shares;
        self.share_ownership.insert(&account_id, &new_total);
        self.shares_outstanding -= new_shares;

        env::log(format!("Account '{}' bought {} shares successfully.", account_id, new_shares).as_bytes());
        env::log(format!("Account '{}' now owns {} shares.", account_id, new_total).as_bytes());
        env::log(format!("New outstanding shares available to buy: {}", self.shares_outstanding).as_bytes());
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn buy_shares() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();
        
        // successful buy
        contract.buy_shares(1_000_000_000);
    }
}
