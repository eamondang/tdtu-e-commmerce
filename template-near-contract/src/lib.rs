use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId, Balance, PanicOnDefault, Promise, env};

pub type JobId = String;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pub owner_id: AccountId,
  pub jobs_per_client: UnorderedMap<AccountId,Vec<Job>>,
  pub jobs_per_dev: UnorderedMap<AccountId,Vec<Job>>,
  pub all_jobs: UnorderedMap<u128,Job>,
  pub job_by_id: LookupMap<JobId,Job>,
  pub all_devs: UnorderedMap<u128,Freelancer>,
  pub dev_by_id: LookupMap<AccountId,Freelancer>,
  pub all_clients: UnorderedMap<u128,Client>,
  pub client_by_id: LookupMap<AccountId,Client>,
  pub total_jobs: u128,
  pub total_devs: u128,
  pub total_clients: u128
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn init() -> Self {
    Self {
      owner_id: env::signer_account_id(),
      jobs_per_client: UnorderedMap::new(b"jobs_per_client".try_to_vec().unwrap()),
      jobs_per_dev: UnorderedMap::new(b"jobs_per_dev".try_to_vec().unwrap()),
      all_jobs: UnorderedMap::new(b"all_jobs".try_to_vec().unwrap()),
      job_by_id: LookupMap::new(b"job_by_id".try_to_vec().unwrap()),
      all_devs: UnorderedMap::new(b"all_devs".try_to_vec().unwrap()),
      dev_by_id: LookupMap::new(b"dev_by_id".try_to_vec().unwrap()),
      all_clients: UnorderedMap::new(b"all_clients".try_to_vec().unwrap()), 
      client_by_id: LookupMap::new(b"client_by_id".try_to_vec().unwrap()), 
      total_jobs: 0,
      total_devs: 0,
      total_clients: 0
    }
  }
}

// Define the freelancer structure
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Freelancer{
  pub dev_id: AccountId,
  pub name: String,
  pub email: String,
  pub phone_number:String
}

// Define the client structure
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Client{
  pub client_id: AccountId,
  pub name: String,
  pub email: String,
  pub phone_number: String
}

// Define the job structure
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
  pub job_id: JobId,
  pub name: String,
  pub desc: String,
  pub reward: Balance,
  pub state: String
}

pub trait OutSourcing {
  // Đăng ký làm freelancer.
  fn register_executor(&mut self, name: String, email: String, phone_number: String) -> Freelancer;
  // Đăng ký để làm người giao job.
  fn register_client(&mut self, name: String, email: String, phone_number: String) -> Client;
  // Client -> Tạo Jobs
  fn create_job(&mut self, job_id: JobId ,name: String, desc: String, reward: Balance, state: String) -> Job;
  // Freelancer -> Take.
  fn take_job(&mut self, job_id: JobId) -> Job;
  // Update
  fn update_job(&mut self, job_id: JobId, state: String, dev_id: AccountId) -> Job;
  // Payment
  fn payment(&self, price: Balance, dev_id: AccountId, job_id: JobId) -> Promise;
  // View
  fn view_all_jobs(&self) -> Vec<Job>;

  fn view_job_by_id(&self, id: JobId) -> Job;
}

// Nhớ là phân insert,

// Implement the contract structure
#[near_bindgen]
impl OutSourcing for Contract {

  fn register_executor(&mut self, name: String, email: String, phone_number: String) -> Freelancer {
    let id = env::signer_account_id();
    assert!(!self.dev_by_id.contains_key(&id), "Developer Account already existed!");
    let dev: Freelancer = Freelancer { dev_id: id.clone(), name: name, email: email, phone_number: phone_number };
    self.dev_by_id.insert(&id, &dev);
    self.total_devs += 1;
    self.all_devs.insert(&self.total_devs, &dev);
    dev 
  }

  fn register_client(&mut self, name: String, email: String, phone_number: String) -> Client {
    let id: AccountId = env::signer_account_id();
    assert!(!self.client_by_id.contains_key(&id), "Client Account already existed!");
    let client: Client = Client { client_id: id.clone(), name: name, email: email, phone_number: phone_number };
    self.client_by_id.insert(&id, &client);
    self.total_clients +=1;
    self.all_clients.insert(&self.total_clients, &client);
    client 
  }

  fn create_job(&mut self, job_id: JobId ,name: String, desc: String, reward: Balance, state: String) -> Job {
    let id: AccountId = env::signer_account_id();
    assert!(self.client_by_id.contains_key(&id), "Client Account doesn't exist!");
    let job: Job = Job { job_id: job_id.clone(), name: name, desc: desc, reward: reward, state: state };
    self.job_by_id.insert(&job_id, &job);
    self.total_jobs+=1;
    self.all_jobs.insert(&self.total_jobs, &job);
    let mut jobs_set: Vec<Job> = self.jobs_per_client.get(&id).unwrap_or_else(|| Vec::new()); 
    jobs_set.push(job.clone());
    self.jobs_per_client.insert(&id, &jobs_set);
    job
  }

  fn take_job(&mut self, job_id: JobId) -> Job {
    let id: AccountId = env::signer_account_id();
    assert!(self.job_by_id.contains_key(&job_id), "Job doesn't exist!");
    let job: Job = self.job_by_id.get(&job_id).unwrap();
    assert!(!(job.state == "Completed"), "Job completed");
    let mut jobs_set: Vec<Job> = self.jobs_per_dev.get(&id).unwrap_or_else(|| Vec::new());
    jobs_set.push(job.clone());
    self.jobs_per_dev.insert(&id, &jobs_set);
    job
  }

  fn update_job(&mut self, job_id: JobId, state: String, dev_id: AccountId) -> Job {
    let id: AccountId = env::signer_account_id();
    assert!(self.client_by_id.contains_key(&id), "Client Account doesn't exist!");
    let mut job = self.job_by_id.get(&job_id).unwrap();
    job.state = state;
    self.job_by_id.insert(&job_id, &job);
    //find index job_by_dev and job_by_client and all_jobs

    //update job in all_jobs
    let mut index_job:u128 = 0;
    for i in 1..self.all_jobs.len() + 1 {
      if let Some(job_found) = self.all_jobs.get(&(i as u128)){
        if job_found.job_id == job.job_id {
          index_job = i as u128;
          break;
        }
      }
    }
    self.all_jobs.insert(&index_job, &job);

    //update job in job_per_dev
    let mut dev_job: Vec<Job> = self.jobs_per_dev.get(&dev_id).unwrap_or_else(|| Vec::new());
    for i in 0..dev_job.len() {
      if let Some(dev_job_found) = dev_job.get(i.clone()){
        if dev_job_found.job_id == job.job_id {
          dev_job.insert(i.clone(), job.clone());
          break;
        }
      }
    }
    self.jobs_per_dev.insert(&dev_id, &dev_job);

    //update job in job_per_client
    let mut client_job: Vec<Job> = self.jobs_per_client.get(&id).unwrap_or_else(|| Vec::new());
    for i in 0..client_job.len() {
      if let Some(client_job_found) = client_job.get(i.clone()){
        if client_job_found.job_id == job.job_id {
          client_job.insert(i.clone(), job.clone());
          break;
        }
      }
    }
    self.jobs_per_client.insert(&id, &client_job);

    job
  }

  fn view_all_jobs(&self) -> Vec<Job> {
    let mut jobs: Vec<Job> = Vec::new();
    for i in 1..self.all_jobs.len() + 1 {
      if let Some(job) = self.all_jobs.get(&(i as u128)){
        jobs.push(job);
      }
    }
    jobs
  }

  fn view_job_by_id(&self, id: JobId) -> Job {
    self.job_by_id.get(&id).unwrap()
  }

  fn payment(&self, price: Balance, dev_id: AccountId, job_id: JobId) -> Promise {
    // Assert!
    // Price == env::attached_deposit();
    let job_payment = self.view_job_by_id(job_id);
    let price_job = job_payment.reward;
    let price_new:u128 = price*10u128.pow(24);
    assert_eq!(price_job, price_new, "Not Correct price");
    Promise::new(dev_id).transfer(price)
  }
}
