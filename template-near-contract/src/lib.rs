use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, PanicOnDefault, Timestamp};
use near_sdk::collections::UnorderedMap;

pub trait OutSourcing {
  fn new() -> Self;
  // Đăng ký để làm người giao job.
  fn register_freelancer(&mut self, name: String) -> ();
  fn register_client(&mut self, name: String) -> ();
  // Client -> Tạo Jobs
  fn create_job(&mut self, name: String, 
    desc: String, 
    payment_amount: Balance, 
    skills: String, 
    experience: u32) -> ();
  // Freelancer -> Take.
  fn take_job(&mut self, job_id: u64) -> ();
  // Count
  fn freelancer_count(&self) -> u64;
  fn client_count(&self) -> u64;
  fn job_count(&self) -> u64;
  // View
  fn view_job_by_id(&self, id: u64) -> Job;
  fn list_all_jobs(&self) -> Vec<Job>;
  fn list_all_clients(&self) -> Vec<AccountId>;
  // fn view_client_by_id
  fn list_all_freelancers(&self) -> Vec<AccountId>;
  // fn view_freelancer_by_id
  fn complete_job(&mut self, job_id: u64) -> Promise;
  // Update
  fn update_job(
    &mut self,
    id: u64, 
    name: Option<String>, 
    desc: Option<String>, 
    payment_amount: Option<Balance>, 
    experience: Option<u32>,
    skills: Option<String>, 
  );
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
  pub id: u64,
  pub name: String,
  pub client_id: AccountId,
  pub desc: String,
  pub skills: String,
  pub experience: u32,
  pub freelancer_id: Option<AccountId>,
  pub status: JobStatus,
  pub payment_amount: Balance,
}
#[derive(BorshDeserialize, BorshSerialize, Serialize,Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountStatus {
  pub name: String,
  pub created_time: Timestamp,
  pub job_related: Vec<u64>
}

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum JobStatus {
  Created,
  InProgress,
  Completed,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OutsourcingPlatform {
  owner_id: AccountId,
  pub freelancers: UnorderedMap<AccountId, AccountStatus>,
  pub clients: UnorderedMap<AccountId, AccountStatus>,
  pub jobs: UnorderedMap<u64, Job>,
  pub job_counter: u64,
}

#[near_bindgen]
impl OutSourcing for OutsourcingPlatform {
    #[init]
    fn new() -> Self {
        Self {
            owner_id: env::signer_account_id(),
            freelancers: UnorderedMap::new(b"freelancers".try_to_vec().unwrap()),
            clients: UnorderedMap::new(b"clients".try_to_vec().unwrap()),
            jobs: UnorderedMap::new(b"jobs".try_to_vec().unwrap()),
            job_counter: 0,
        }
    }

    fn register_freelancer(&mut self, name: String) -> () {
        let freelancer_id = env::predecessor_account_id();
        if self.freelancers.get(&freelancer_id).is_some() {
          println!("Sorry but you already registed as a freelancer");
        } else {
          let account_status = AccountStatus {
            name,
            created_time: env::block_timestamp_ms(),
            job_related: Vec::new(),
          };
          self.freelancers.insert(&freelancer_id, &account_status);
        }
    }

    fn list_all_freelancers(&self) -> Vec<AccountId> {
      let mut all_freelancers: Vec<AccountId> = Vec::new();
      // let unordered_clone =  &self.freelancers;
      for (key, value) in self.freelancers.iter() {
        all_freelancers.push(key);
    }
      all_freelancers
    }

    fn register_client(&mut self, name: String) {
        let client_id = env::predecessor_account_id();
        if self.clients.get(&client_id).is_some() {
          println!("Sorry but you already registed as a client");
        }
        else {
          let account_status = AccountStatus {
            name,
            created_time: env::block_timestamp_ms(),
            job_related: Vec::new(),
          };
          self.clients.insert(&client_id, &account_status);
        }
    }

    fn list_all_clients(&self) -> Vec<AccountId> {
      let mut all_clients: Vec<AccountId> = Vec::new();
      // let unordered_clone =  &self.clients;
      for (key, value) in self.clients.iter() {
        all_clients.push(key);
    }
      all_clients
    }

    fn create_job(&mut self, name: String, desc: String, payment_amount: Balance, skills: String, experience: u32) -> () {
        let client = env::predecessor_account_id();
        self.job_counter += 1;
        // let client_clone = client.clone();
        // let status = self.clients.get(&client_clone);

        let job = Job {
            id: self.job_counter,
            name,
            client_id: client,
            skills,
            experience,
            desc,
            freelancer_id: None,
            status: JobStatus::Created,
            payment_amount,
        };


        self.jobs.insert(&self.job_counter, &job);
        
    } 

    fn update_job(
      &mut self, 
      id: u64, 
      name: Option<String>, 
      desc: Option<String>, 
      payment_amount: Option<Balance>, 
      experience: Option<u32>,
      skills: Option<String>  
    ) {
      let mut job = self.view_job_by_id(id);
      assert_eq!(job.client_id.clone(), env::signer_account_id(), "You don't have permission to edit this job!");
      
      if let Some(name) = name { job.name = name };
      if let Some(payment_amount) = payment_amount { job.payment_amount = payment_amount };
      if let Some(desc) = desc { job.desc = desc };
      if let Some(experience) = experience { job.experience = experience };
      if let Some(skills) = skills { job.skills = skills };
  
      self.jobs.insert(&job.id, &job);
    }
  

    fn take_job(&mut self, job_id: u64) {
        let freelancer_id = env::predecessor_account_id();
        let job = self.jobs.get(&job_id).expect("Job does not exist");
        assert_eq!(job.status, JobStatus::Created, "Job is not available");
        assert_eq!(job.status, JobStatus::Completed, "Job is already done");

        let mut updated_job = job;
        updated_job.status = JobStatus::InProgress;
        updated_job.freelancer_id = Some(freelancer_id);
        self.jobs.insert(&job_id, &updated_job);
    }

    fn freelancer_count(&self) -> u64{
      self.freelancers.len()
    }

    fn client_count(&self) -> u64{
      self.clients.len()
    }

    fn job_count(&self) -> u64{
      self.jobs.len()
    }

    fn view_job_by_id(&self, id: u64) -> Job {
      self.jobs.get(&id).expect("There is no job")
    }

    fn list_all_jobs(&self) -> Vec<Job> {
        self.jobs.values().collect()
    }
    fn complete_job(&mut self, job_id: u64) -> Promise {
        // let freelancer_id = env::predecessor_account_id();
        let job = self.jobs.get(&job_id).expect("Job does not exist");
        assert_eq!(job.payment_amount, env::attached_deposit() / 10u128.pow(24), "You dont have enough money");
        assert_eq!(
            job.status, JobStatus::InProgress,
            "Job is not in progress"
        );
        self.jobs.get(&job_id).map(|mut job| {
            job.status = JobStatus::Completed;
        });
        let payment_amount = job.payment_amount;
        Promise::new(env::signer_account_id()).transfer(payment_amount)
    }

}