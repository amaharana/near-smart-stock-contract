// TODO turn on halt on warning

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::Promise;
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

// conversion factor from NEAR token to yoctoNear units
pub const YOCTO_MULTIPLIER: u128 = 1_000_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  ticker: String,
  total_shares: u32,
  shares_outstanding: u32,

  // NEAR tokens required to buy one share
  // TODO: Fetch current FIAT price using chainlink oracle and convert to NEAR tokens
  price_per_share: u32,

  share_ownership: LookupMap<String, u32>,
}

impl Default for Contract {
  fn default() -> Self {
    panic!("SHOULD_INIT_BEFORE_USE");
  }
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new(ticker: String, total_shares: u32, price_per_share: u32) -> Self {
    assert!(!env::state_exists(), "ALREADY_INITIALIZED");
    assert!(
      env::current_account_id() == env::signer_account_id(),
      "INIT_ONLY_BY_OWNER"
    );

    Self {
      ticker: ticker,
      total_shares: total_shares,
      price_per_share: price_per_share,

      // nothing has been sold yet
      shares_outstanding: total_shares,

      // TODO: what does b"a" signify here?
      // to keep track of who has bought how many shares
      share_ownership: LookupMap::new(b"a"),
    }
  }

  #[payable]
  pub fn buy_shares(&mut self, num_shares: u32) {
    let account_id = env::signer_account_id();

    if num_shares > self.shares_outstanding || num_shares == 0 {
      panic!("ERR_INVALID_BUY_QUANTITY");
    }

    if env::attached_deposit() < u128::from(num_shares * self.price_per_share) * YOCTO_MULTIPLIER
    {
      panic!("ERR_INSUFFICIENT_DEPOSIT");
    }

    let current_shares = self.share_ownership.get(&account_id).unwrap_or_default();
    let new_total = current_shares + num_shares;
    self.share_ownership.insert(&account_id, &new_total);
    self.shares_outstanding -= num_shares;

    env::log(
      format!(
        "Account {} bought {} {} shares successfully.",
        account_id, num_shares, self.ticker
      )
      .as_bytes(),
    );
    env::log(
      format!(
        "Account {} now owns {} {} shares.",
        account_id, new_total, self.ticker
      )
      .as_bytes(),
    );
    env::log(
      format!(
        "New outstanding {} shares available to buy: {}",
        self.ticker, self.shares_outstanding
      )
      .as_bytes(),
    );
  }

  pub fn sell_shares(&mut self, num_shares: u32) -> Promise {
    let account_id = env::signer_account_id();
    let same_account_id = env::signer_account_id();

    let current_shares = self.share_ownership.get(&account_id).unwrap_or_default();

    if num_shares > current_shares || num_shares == 0 {
      panic!("ERR_INVALID_SELL_QUANTITY");
    }

    let new_total = current_shares - num_shares;
    let sale_proceeds = u128::from(num_shares * self.price_per_share) * YOCTO_MULTIPLIER;

    self.share_ownership.insert(&account_id, &new_total);
    self.shares_outstanding += num_shares;

    // TODO: Shouldn't this be displayed only after the transfer Promise has completed successfully? How do we check/get notified when the promise executes successfully?
    env::log(
      format!(
        "Account {} sold {} {} shares successfully.",
        account_id, self.ticker, num_shares
      )
      .as_bytes(),
    );
    env::log(
      format!(
        "Account {} now owns {} {} shares.",
        account_id, self.ticker, new_total
      )
      .as_bytes(),
    );
    env::log(
      format!(
        "New outstanding {} shares available to buy: {}",
        self.ticker, self.shares_outstanding
      )
      .as_bytes(),
    );

    // TODO: What about gas? Should the gas be subtracted from sale proceeds and passed to this promise?
    // TODO: "Lock" these funds somehow until the promise is executed successfully. How do we check/get notified when the promise executes successfully?
    Promise::new(same_account_id).transfer(sale_proceeds)
  }

  pub fn get_shares_outstanding(&self) -> u32 {
    self.shares_outstanding
  }

  pub fn get_total_shares(&self) -> u32 {
    self.total_shares
  }

  pub fn get_shares_owned(&self) -> u32 {
    let account_id = env::signer_account_id();
    self.share_ownership.get(&account_id).unwrap_or_default()
  }

  pub fn get_price_per_share(&self) -> u32 {
    self.price_per_share
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
      signer_account_id: "alice_near".to_string(),
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

  // test for default initialization
  // test for init only once

  #[test]
  #[should_panic(expected = "SHOULD_INIT_BEFORE_USE")]
  fn default_contract() {
    let context = get_context(vec![], false);
    testing_env!(context);
    Contract::default();
  }

  #[test]
  #[should_panic(expected = "INIT_ONLY_BY_OWNER")]
  fn init_by_non_owner() {
    let mut context = get_context(vec![], false);
    context.signer_account_id = "bob_near".to_string();
    testing_env!(context);

    Contract::new("MYCOMPANY".to_string(), 1_000_000_000, 2);
  }

  #[test]
  #[should_panic(expected = "ERR_INVALID_BUY_QUANTITY")]
  fn buy_shares_more_than_outstanding() {
    let context = get_context(vec![], false);
    testing_env!(context);
    let mut contract = Contract::new("MYCOMPANY".to_string(), 1_000_000_000, 2);
    let buy_quantity = contract.get_shares_outstanding() + 1;

    contract.buy_shares(buy_quantity);
  }

  #[test]
  fn buy_shares_successfully() {
    let mut context = get_context(vec![], false);
    context.attached_deposit = 2 * YOCTO_MULTIPLIER * 420;
    testing_env!(context);
    let mut contract = Contract::new("MYCOMPANY".to_string(), 1_000_000_000, 2);

    // haven't bought anything yet
    assert_eq!(contract.get_shares_owned(), 0);

    contract.buy_shares(20);
    contract.buy_shares(400);
    assert_eq!(contract.get_shares_owned(), 420);

    let expected_shares_outstanding = contract.get_total_shares() - 420;
    assert_eq!(
      contract.get_shares_outstanding(),
      expected_shares_outstanding
    );
  }

  #[test]
  #[should_panic(expected = "ERR_INVALID_SELL_QUANTITY")]
  fn sell_shares_more_than_owned() {
    let mut context = get_context(vec![], false);
    context.attached_deposit = 2 * YOCTO_MULTIPLIER * 290;
    testing_env!(context);    
    let mut contract = Contract::new("MYCOMPANY".to_string(), 1_000_000_000, 2);

    contract.buy_shares(290);
    contract.sell_shares(291);
  }

  #[test]
  fn sell_shares_successfully() {
    let mut context = get_context(vec![], false);
    context.attached_deposit = 2 * YOCTO_MULTIPLIER * 286;
    testing_env!(context);
    let mut contract = Contract::new("MYCOMPANY".to_string(), 1_000_000_000, 2);

    contract.buy_shares(20);
    contract.buy_shares(81);
    contract.buy_shares(185);
    contract.sell_shares(52);
    assert_eq!(contract.get_shares_owned(), 234);

    let expected_shares_outstanding = contract.get_total_shares() - 234;
    assert_eq!(
      contract.get_shares_outstanding(),
      expected_shares_outstanding
    );

    // TODO: insert check that sale proceeds were transferred to caller successfully
  }
}
