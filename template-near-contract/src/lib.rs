use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::{UnorderedMap, LookupMap};
pub type JobId = String;


// #[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct User {
//   pub user_id: AccountId,
//   pub role: String,
// }



// Define the contract structure
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub owner_id: AccountId,
  pub user: LookupMap<AccountId, String>,
  pub job_per_executor: UnorderedMap<AccountId, Vec<Job>>,
  pub job_per_client: UnorderedMap<AccountId, Vec<Job>>,
  pub job_by_id: LookupMap<JobId, Job>,
  pub jobs: UnorderedMap<u128, Job>,
  pub total_jobs: u128,
  pub total_users: u128,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn init() -> Self {
    Self {
      owner_id: env::signer_account_id(),
      job_per_executor: UnorderedMap::new(StorageKey::JobPerExecutorKey.try_to_vec().unwrap()),
      job_per_client: UnorderedMap::new(StorageKey::JobPerClientKey.try_to_vec().unwrap()),
      job_by_id: LookupMap::new(b"job by id".try_to_vec().unwrap()),
      jobs: UnorderedMap::new(b"jobs".try_to_vec().unwrap()),
      user: LookupMap::new(b"user".try_to_vec().unwrap()),
      total_jobs: 0,
      total_users: 0,
    }
  }
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum StorageKey {
  JobPerExecutorKey,
  JobPerClientKey,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
  pub job_id: JobId,
  pub client: AccountId,
  pub executor: AccountId,
  pub name: String,
  pub description: String,
  pub price: Balance,
  pub status: String,
}
pub trait OutSourcing {
  // Đăng ký làm freelancer.
  fn register_executor(&mut self);
  // Đăng ký để làm người giao job.
  fn register_client(&mut self);
  // Client -> Tạo Jobs
  fn create_job(&mut self, name: String, description: String, price: Balance);
  // Freelancer -> Take.
  fn take_job(&mut self, name: String, owner: AccountId);
  // Update
  fn update_job(&mut self, name: String, description: String, price: Balance, status: String);
  // Payment
  fn payment(&mut self, name_job: String, price: Balance) -> Promise;
  // View
  fn view_all_jobs(&self) -> Vec<Job>;
  fn view_job_by_id(&self, job_id: JobId) -> Job;
}
// Nhớ là phân insert,

// Implement the contract structure
#[near_bindgen]
impl OutSourcing for Contract {
  fn register_executor(&mut self) {
    let owner = env::signer_account_id();
    // assert!(!self.job_per_executor.contains_key(&owner), "Executor already exists");
    self.user.insert(&owner, &String::from("Executor"));
    self.total_users = self.total_users + 1;
  }
  fn register_client(&mut self) {
    let owner = env::signer_account_id();
    // assert!(!self.job_per_client.contains_key(&owner), "Client already exists");
    self.user.insert(&owner, &String::from("Client"));
    self.total_users = self.total_users + 1;
  }
  fn create_job(&mut self, name: String, description: String, price: Balance) {
    let owner = env::signer_account_id();
    assert!(self.user.contains_key(&owner), "User does not exist");
    assert_eq!(self.user.get(&owner).unwrap(), String::from("Executor"), "This account is not a freelancer");
    let a = self.job_per_executor.get(&owner).unwrap().clone();
    for i in 0..a.len() {
      if a[i].name == name {
        print!("Job already exists");
        return
      }
    }
    let job = Job {
      job_id: self.total_jobs.clone().to_string(),
      client: env::signer_account_id(),
      executor: env::signer_account_id(),
      name,
      description,
      price,
      status: String::from("Pending"),
    };
    self.job_by_id.insert(&self.total_jobs.to_string(), &job);
    self.total_jobs = self.total_jobs + 1;
    let mut job_set: Vec<Job> = self.job_per_executor.get(&owner).unwrap_or_else(|| Vec::new());
    job_set.push(job.clone());
    self.job_per_executor.insert(&owner, &job_set);
  }

  fn take_job(&mut self, name: String, owner: AccountId) {
    let owner = owner;
    let mut a = self.job_per_executor.get(&owner).unwrap().clone();
    for i in 0..a.len() {
      if a[i].name == name {
        a[i].client = env::signer_account_id();
        a[i].status = String::from("Processing");
        self.job_per_client.insert(&env::signer_account_id(), &a);
        self.job_per_executor.insert(&owner, &a);
        self.job_by_id.insert(&a[i].job_id, &a[i]);
        self.jobs.insert(&(i as u128), &a[i]);
        break
      }
    }
  }
  fn update_job(&mut self, name: String, description: String, price: Balance, status: String) {
    assert!(self.user.contains_key(&env::signer_account_id()), "User does not exist");
    assert!(self.user.get(&env::signer_account_id()).unwrap() == String::from("Executor"), "This account is not a freelancer");
    let owner = env::signer_account_id();
    let mut a = self.job_per_executor.get(&owner).unwrap().clone();
    for i in 0..a.len() {
      if a[i].name == name {
        let id_job = a[i].job_id.clone();
        a[i].description = description.clone();
        a[i].price = price.clone();
        a[i].status = status.clone();
        self.job_per_executor.insert(&owner, &a);
        let mut b = self.job_per_client.get(&a[i].client).unwrap().clone();
        for j in 0..b.len() {
          if b[j].job_id == id_job {
            b[j].description = description.clone();
            b[j].price = price.clone();
            b[j].status = status.clone();
            self.job_per_client.insert(&a[i].client, &b);
            self.job_by_id.insert(&b[j].job_id, &b[j]);
            self.jobs.insert(&(j as u128), &b[j]);
            break
          }
        }
        break
      }
    }
  }
  fn view_all_jobs(&self) -> Vec<Job> {
    let mut all_jobs: Vec<Job> = Vec::new();
    for i in 0..self.jobs.len() {
      all_jobs.push(self.job_by_id.get(&i.to_string()).unwrap().clone());
    }
    all_jobs
  }
  fn view_job_by_id(&self, job_id: JobId) -> Job {
    self.job_by_id.get(&job_id).unwrap()
  }

  #[payable]
  fn payment(&mut self, name_job: String, price: Balance) -> Promise {
    assert!(self.user.contains_key(&env::signer_account_id()), "User does not exist");
    assert!(self.jobs.len() > 0, "No jobs");
    let mut client_name = env::signer_account_id().to_string();
    let mut price = price;
    for i in 0..self.jobs.len() {
      let a = self.job_by_id.get(&i.to_string()).unwrap().clone();
      if a.name == name_job {
        price = a.price.clone() * u128::pow(10, 24);
        self.update_job(a.name, a.description, a.price, "Completed".to_string());
        client_name = a.client.to_string();
      }
    }
    Promise::new(client_name.parse().unwrap()).transfer(price)
  }
}


