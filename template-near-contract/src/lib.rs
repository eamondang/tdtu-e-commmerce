use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::{near_bindgen, AccountId, Balance, PanicOnDefault, Promise, env};

pub type JobId=String;
pub type FreelancerId=String;
pub type ClientId=String;
// Define the contract structure
// 1 executor, 1 owner có nhiều jobs
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
  pub job_id:JobId,
  pub job_name:String,
  pub client_id:String,
}
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Freelancer{
  pub freelancer_id:FreelancerId,
  pub freelancer_name:String,
  pub freelancer_transfer:String,
  pub freelancer_jobs:Vec<JobId>,
}
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct  Client{
  pub client_id: String,
  pub client_name: String,
  pub client_transfer:String,
  // pub client_jobs:Vec<JobId>,
}
// owner nhận tiền trực tiếp từ client và chia tiền cho freelance
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pub owner_id: AccountId,
  //Jobs
  pub all_jobs:UnorderedMap<u128,Job>,
  pub job_by_id:LookupMap<JobId,Job>,
  pub jobs_by_executor:LookupMap<FreelancerId,Vec<JobId>>,
  pub jobs_by_owner:LookupMap<AccountId,Vec<Job>>,
  pub total_jobs:u128,
  //Freelancers
  pub all_freelancers:UnorderedMap<u128,Freelancer>,
  pub freelancer_by_id:LookupMap<FreelancerId,Freelancer>,
  pub total_freelancers:u128,
  //Clients
  pub all_clients:UnorderedMap<u128,Client>,
  pub client_by_id:LookupMap<ClientId,Client>,
  pub total_client:u128
}
pub trait OutSourcing {
  fn init()->Self;
  // Đăng ký làm freelancer.
  fn register_executor(&mut self,freelancer_id:FreelancerId,freelancer_name:String,freelancer_transfer:String)->Freelancer;
  // Đăng ký để làm người giao job.
  fn register_client(&mut self,client_id:ClientId,client_name:String, client_transfer:String)->Client;
  // Client -> Tạo Jobs
  fn create_job(&mut self, job_id:JobId,job_name:String,client_id:ClientId)->Job;
  // Freelancer -> Take.
  fn take_job(&mut self,freelancer_id:FreelancerId,job:Vec<JobId>);
  // Update
  fn update_job(&mut self,job_id:JobId,job_name:String,client_id:ClientId);
  // Payment
  fn payment(price: Balance) -> Promise;
  // View
  fn view_all_jobs(&self) -> Vec<String>;
  fn view_job_by_id(&self,job_id:JobId)->Job;
}

// Nhớ là phân insert,

// Implement the contract structure
#[near_bindgen]
impl OutSourcing for Contract {
  #[init]
  fn init() -> Self
  {
    Self{
      owner_id: env::signer_account_id(),
      all_jobs: UnorderedMap::new(b"all jobs".try_to_vec().unwrap()),
      job_by_id:LookupMap::new(b"job by id".try_to_vec().unwrap()),
      jobs_by_executor:LookupMap::new(b"jobs by executor".try_to_vec().unwrap()),
      jobs_by_owner:LookupMap::new(b"jobs by owner".try_to_vec().unwrap()),
      total_jobs:0,
      all_freelancers:UnorderedMap::new(b"all freelancers".try_to_vec().unwrap()),
      freelancer_by_id:LookupMap::new(b"freelancer by id".try_to_vec().unwrap()),
      total_freelancers:0,
      all_clients:UnorderedMap::new(b"all clients".try_to_vec().unwrap()),
      client_by_id:LookupMap::new(b"client by id".try_to_vec().unwrap()),
      total_client:0
    }
  }
  fn register_executor(
    &mut self,
    freelancer_id:FreelancerId,
    freelancer_name:String,
    freelancer_transfer:String
  )->Freelancer {
    let jobs_freelance:Vec<JobId>=Vec::new();
    let freelance=Freelancer{freelancer_id:freelancer_id.clone(),
      freelancer_name,freelancer_transfer,
      freelancer_jobs:jobs_freelance};
    let total=self.total_freelancers+1;
    self.freelancer_by_id.insert(&freelancer_id, &freelance);
    self.all_freelancers.insert(&total, &freelance);
    freelance
  }
  fn register_client(
    &mut self,
    client_id:ClientId,
    client_name:String, 
    client_transfer:String
  )->Client {
    let total=self.total_client+1;
    let client=Client{client_id: client_id.clone(),client_name,client_transfer};
    self.client_by_id.insert(&client_id, &client);
    self.all_clients.insert(&total, &client);
    client
  }
  fn create_job(&mut self,job_id:JobId,job_name:String, client_id:ClientId)->Job {
    let total=self.total_jobs+1;
    let job=Job{job_id:job_id.clone(),job_name,client_id:client_id.clone()};
    self.job_by_id.insert(&job_id,&job);
    // let mut jobs_set:Vec<Job>=self.jobs_by_owner.get(&client_id).unwrap_or_else(|| Vec::new());
    self.all_jobs.insert(&total,&job);
    job
  }
  // Freelancer -> Take.
  fn take_job(&mut self,freelancer_id:FreelancerId,job:Vec<JobId>) {
    self.jobs_by_executor.insert(&freelancer_id,&job);
    let mut freelance:Freelancer=self.freelancer_by_id.get(&freelancer_id).unwrap();
    freelance.freelancer_jobs.extend(job);
    self.freelancer_by_id.insert(&freelancer_id, &freelance);
  }
  fn update_job(&mut self,job_id:JobId,job_name:String,client_id:ClientId) {
    let mut job=self.view_job_by_id(job_id.clone());
    job.job_name=job_name;
    job.client_id=client_id;
    self.job_by_id.insert(&job_id, &job);
  }
  fn view_all_jobs(&self) -> Vec<String> {
    let mut all_jobs:Vec<String>=Vec::new();
    for (key,job) in self.all_jobs.iter(){
      all_jobs.push(job.job_id);
    }
    all_jobs
  }
  fn view_job_by_id(&self,job_id:JobId)->Job {
    self.job_by_id.get(&job_id).unwrap()
  }
  fn payment(price: Balance) -> Promise {
    // Assert!
    // Price == env::attached_deposit();
    Promise::new("traha1.testnet".parse().unwrap()).transfer(price)
  }
}
