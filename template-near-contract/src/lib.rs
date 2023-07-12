use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, Balance, PanicOnDefault, Promise};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pub owner_id: AccountId,
}

pub struct Job {}

pub trait OutSourcing {
  // Đăng ký làm freelancer.
  fn register_executor();
  // Đăng ký để làm người giao job.
  fn register_client();
  // Client -> Tạo Jobs
  fn create_job();
  // Freelancer -> Take.
  fn take_job();
  // Update
  fn update_job();
  // Payment
  fn payment(price: Balance) -> Promise;
  // View
  fn view_all_jobs() -> Vec<String>;
  fn view_job_by_id();
}

// Nhớ là phân insert,

// Implement the contract structure
#[near_bindgen]
impl OutSourcing for Contract {
  fn register_executor() {
    todo!()
  }
  fn register_client() {
    todo!()
  }
  fn create_job() {
    todo!()
  }
  fn take_job() {
    todo!()
  }
  fn update_job() {
    todo!()
  }
  fn view_all_jobs() -> Vec<String> {
    vec![]
  }
  fn view_job_by_id() {}
  fn payment(price: Balance) -> Promise {
    // Assert!
    // Price == env::attached_deposit();
    Promise::new("eamondev.testnet".parse().unwrap()).transfer(price)
  }
}
