use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, AccountId, Balance, PanicOnDefault, Promise, Timestamp, env};
use near_sdk::serde::{Deserialize, Serialize};

pub type JobId = u128;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Status {
  Available,
  Taken,
  Expired,
  Done
}
// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pub owner_id: AccountId,
  pub user_list: UnorderedMap<AccountId, User>,
  pub job_list: UnorderedMap<JobId, Job>,
  pub total_user: u128,
  pub total_job: u128
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
  pub id: AccountId,
  pub name: String,
  pub occupation: String,
  pub company: String,
  pub desc: String,
  pub given_job: u128,
  pub taken_job: u128,
  pub taken_list: HashMap<u128, Job>,
  pub given_list: HashMap<u128, Job>,
  pub created_date: Timestamp
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
  pub job_id: JobId,
  pub job_name: String,
  pub job_desc: String,
  pub job_salary: Balance,
  pub created_date: Timestamp,
  pub exp: String,
  pub client: AccountId,
  pub taken_by: Option<AccountId>,
  pub status: Status,
}

pub trait OutSourcing {
  fn init() -> Self;
  // Đăng ký để làm người giao job.
  fn register_user(
    &mut self,
    name: String,
    occupation: String, 
    company: String, 
    desc: String,
  ) -> User;
  // Client -> Tạo Jobs
  fn create_job(&mut self, name: String, desc: String, salary: Balance, exp: String) -> Job;
  // Freelancer -> Take.
  fn take_job(&mut self, job_id: JobId) -> Job;
  // Update
  fn update_job(
    &mut self,
    id: JobId, 
    name: Option<String>, 
    desc: Option<String>, 
    salary: Option<Balance>, 
    exp: Option<String>
  );
  // Payment
  fn payment(&mut self, job_id: JobId, amount: Balance) -> Promise;
  // View
  fn view_all_jobs(&self) -> Vec<Job>;
  fn view_job_by_id(&self, id: JobId) -> Job;
  fn view_all_users(&self) -> Vec<User>;
}

// Nhớ là phân insert,

// Implement the contract structure
#[near_bindgen]
impl OutSourcing for Contract {
  #[init]
  fn init() -> Self {
    Self { 
      owner_id: env::signer_account_id(), 
      total_user: 0, total_job: 0, 
      user_list: UnorderedMap::new(b"user_list".try_to_vec().unwrap()),
      job_list: UnorderedMap::new(b"job_list".try_to_vec().unwrap())
    }
  }

  fn register_user(
    &mut self,
    name: String, 
    occupation: String, 
    company: String,
    desc: String,
  ) -> User {
    let user = User {
      id: env::signer_account_id(),
      name, occupation, company, desc,
      given_job: 0,
      taken_job: 0,
      created_date: env::block_timestamp_ms(),
      taken_list: HashMap::new(),
      given_list: HashMap::new()
    };

    self.user_list.insert(&user.id, &user);

    self.total_user += 1;
    user
  }

  fn create_job(
    &mut self, name: String, desc: String, salary: Balance, exp: String
  ) -> Job {
    if env::account_balance() / 10u128.pow(24) < salary {
      panic!("Not enough money");
    }

    let job = Job {
      job_id: self.total_job.clone(),
      job_name: name.clone(), 
      job_salary: salary,
      job_desc: desc,
      created_date: env::block_timestamp_ms(),
      exp,
      client: env::signer_account_id(),
      status: Status::Available,
      taken_by: None
    };

    self.job_list.insert(&job.job_id, &job);

    self.total_job += 1;

    let user_id = env::signer_account_id();
    let mut user = self.user_list.get(&user_id).expect("There is no user");

    user.given_job += 1;
    user.given_list.insert(user.given_job, self.view_job_by_id(job.job_id.clone()));
    
    job
  }

  fn take_job(&mut self, job_id: JobId) -> Job {
    let user_id = env::signer_account_id();

    let mut user = self.user_list.get(&user_id).expect("There is no user");
    let mut job = self.view_job_by_id(job_id);

    if job.status == Status::Taken {
      panic!("This job has been taken");
    }

    job.status = Status::Taken;
    job.taken_by = Some(user_id.clone());
    self.job_list.insert(&job.job_id.clone(), &job);
    
    user.taken_job += 1;
    user.taken_list.insert(user.taken_job, self.view_job_by_id(job_id));

    self.user_list.insert(&user_id.clone(), &user);
    
    job
  }

  fn update_job(
    &mut self, 
    id: JobId, 
    name: Option<String>, 
    desc: Option<String>, 
    salary: Option<Balance>, 
    exp: Option<String>  
  ) {
    let mut job = self.view_job_by_id(id);
    assert_eq!(job.client.clone(), env::signer_account_id(), "Unauthorized");
    
    if let Some(name) = name { job.job_name = name }
    if let Some(salary) = salary { job.job_salary = salary }
    if let Some(desc) = desc { job.job_desc = desc }
    if let Some(exp) = exp { job.exp = exp }

    self.job_list.insert(&job.job_id, &job);
  }

  fn view_all_jobs(&self) -> Vec<Job> {
    self.job_list.values().collect()
  }

  fn view_job_by_id(&self, id: JobId) -> Job {
    self.job_list.get(&id).expect("There is no job")
  }
  
  #[payable]
  fn payment(&mut self, job_id: JobId, amount: Balance) -> Promise {
    let mut job = self.view_job_by_id(job_id);

    // Assert!
    // Price == env::attached_deposit();
    assert_eq!(amount, env::attached_deposit() / 10u128.pow(24), "Not enough money");
    assert_eq!(job.job_salary, amount, "Not correct amount");
    
    job.status = Status::Done;
    self.job_list.insert(&job_id.clone(), &job);

    Promise::new(env::signer_account_id()).transfer(amount)
  }

  fn view_all_users(&self) -> Vec<User> {
    self.user_list.values().collect()
  }
}
