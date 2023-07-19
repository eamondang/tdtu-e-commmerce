use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, PanicOnDefault};
use near_sdk::collections::UnorderedMap;

// dev-1689678183079-34312263903299


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
  id: u64,
  client_id: AccountId,
  freelancer_id: Option<AccountId>,
  status: JobStatus,
  payment_amount: Balance,
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
pub struct FreelancePlatform {
    freelancers: UnorderedMap<AccountId, bool>,
    clients: UnorderedMap<AccountId, bool>,
    jobs: UnorderedMap<u64, Job>,
    job_counter: u64,
}

#[near_bindgen]
impl FreelancePlatform {
    #[init]
    pub fn new() -> Self {
        Self {

            freelancers: UnorderedMap::new(b"freelancers".try_to_vec().unwrap()),
            clients: UnorderedMap::new(b"clients".try_to_vec().unwrap()),
            jobs: UnorderedMap::new(b"jobs".try_to_vec().unwrap()),
            job_counter: 0,
        }
    }

    pub fn register_freelancer(&mut self) {
        let freelancer_id = env::predecessor_account_id();
        if self.freelancers.get(&freelancer_id).is_some() {
          println!("Sorry but you already registed as a freelancer");
        } else {
          self.freelancers.insert(&freelancer_id, &true);
        }
    }

    pub fn list_all_freelancers(&self) -> Vec<AccountId> {
      let mut all_freelancers: Vec<AccountId> = Vec::new();
      // let unordered_clone =  &self.freelancers;
      for (key, value) in self.freelancers.iter() {
        all_freelancers.push(key);
    }
      all_freelancers
    }

    pub fn register_client(&mut self) {
        let client_id = env::predecessor_account_id();
        if self.clients.get(&client_id).is_some() {
          println!("Sorry but you already registed as a client");
        }
        self.clients.insert(&client_id, &true);
    }

    pub fn list_all_clients(&self) -> Vec<AccountId> {
      let mut all_clients: Vec<AccountId> = Vec::new();
      // let unordered_clone =  &self.clients;
      for (key, value) in self.clients.iter() {
        all_clients.push(key);
    }
      all_clients
    }

    pub fn create_job(&mut self, payment_amount: Balance) {
        let client_id = env::predecessor_account_id();
        self.job_counter += 1;
        let job = Job {
            id: self.job_counter,
            client_id,
            freelancer_id: None,
            status: JobStatus::Created,
            payment_amount,
        };
        self.jobs.insert(&self.job_counter, &job);
    }

    pub fn take_job(&mut self, job_id: u64) {
        let freelancer_id = env::predecessor_account_id();
        let job = self.jobs.get(&job_id).expect("Job does not exist");
        assert_eq!(job.status, JobStatus::Created, "Job is not available");
        assert_eq!(job.status, JobStatus::Completed, "Job is done");

        let mut updated_job = job;
        updated_job.status = JobStatus::InProgress;
        updated_job.freelancer_id = Some(freelancer_id);
        self.jobs.insert(&job_id, &updated_job);
    }

    pub fn complete_job(&mut self, job_id: u64) -> Promise {
        // let freelancer_id = env::predecessor_account_id();
        let job = self.jobs.get(&job_id).expect("Job does not exist");
        assert_eq!(
            job.status, JobStatus::InProgress,
            "Job is not in progress"
        );


        self.jobs.get(&job_id).map(|mut job| {
            job.status = JobStatus::Completed;
        });

        let client_id = job.client_id.clone();
        let payment_amount = job.payment_amount;
        Promise::new(client_id).transfer(payment_amount)
    }

    pub fn list_all_jobs(&self) -> Vec<Job> {
        self.jobs.values().collect()
    }
}