pub mod events;

use crate::events::EventLog;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub client_by_id: LookupMap<AccountId, InfoPerson>,
    pub clients: UnorderedMap<AccountId, InfoPerson>,
    pub exercutors: UnorderedMap<AccountId, InfoPerson>,
    pub exercutor_by_id: LookupMap<AccountId, InfoPerson>,
    pub job_by_id: LookupMap<String, Job>,
    pub jobs: UnorderedMap<String, Job>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, PanicOnDefault, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct InfoPerson {
    pub name: String,
    pub id: AccountId,
}

#[derive(
    BorshDeserialize, BorshSerialize, Deserialize, Serialize, PanicOnDefault, Debug, Clone,
)]
#[serde(crate = "near_sdk::serde")]

pub struct Job {
    pub id: String,
    pub title: String,
    pub desc: String,
    pub wage: Balance,
    pub owner: AccountId,
    pub check_take: bool,
    pub check_finish: bool,
    pub executor_id: AccountId,
}

pub trait OutSourcing {
    // dev-1689661765099-19566568641461
    // cargo make call-self new
    // cargo make call new --account-id nnhoang.testnet
    
    // View
    // cargo make view view_all_jobs
    fn view_all_jobs(&self) -> Vec<Job>;

    // cargo make view view_job_by_id '{"id": "bc"}'
    fn view_job_by_id(&self, id: String) -> Job;

    // Xem tất cả người làm freelancer
    // cargo make view view_all_executors
    fn view_all_executors(&self) -> Vec<InfoPerson>;

    // Xem tất cả người giao việc
    // cargo make view view_all_clients
    fn view_all_clients(&self) -> Vec<InfoPerson>;

    // cargo make view executor_by_id --account-id nnhoang.testnet
    fn executor_by_id(&self) -> InfoPerson;

    // cargo make view client_by_id --account-id nnhoang.testnet
    fn client_by_id(&self) -> InfoPerson;


    // Register
    // cargo make call register_executor '{"name": "Duy"}' --account-id dev-1689661765099-19566568641461
    // Đăng ký làm freelancer.
    fn register_executor(&mut self, name: String) -> InfoPerson;

    // Đăng ký để làm người giao job.
    // cargo make call register_client '{"name": "Hoang"}' --account-id nnhoang.testnet
    fn register_client(&mut self, name: String) -> InfoPerson;

    

    // Client -> Tạo Jobs
    // cargo make call create_job '{"id": "bc", "title": "smartcontract", "desc": "Lap trinh smartcontract", "wage": 2}' --account-id nnhoang.testnet
    fn create_job(&mut self, id: String, title: String, desc: String, wage: Balance) -> Job;

    // Freelancer -> Take.
    // cargo make call take_job '{"id": "bc"}' --account-id nnhoangg.testnet
    fn take_job(&mut self, id: String) -> Job ;

    // Executor finished job
    // cargo make call finish_job '{"id": "bc"}' --account-id nnhoangg.testnet
    fn finish_job(&mut self, id: String)-> Job;

    // Update
    // cargo make call update_job_for_client '{"id": "bc", "title": "Maketing", "desc": "Digital Maketing", "wage": 2}' --account-id nnhoang.testnet
    fn update_job_for_client(
        &mut self,
        id: String,
        title: String,
        desc: String,
        wage: Balance,
    ) -> Job;



    // Payment
    // cargo make call payment '{"id": "bc"}' --account-id nnhoang.testnet --amount 2
    fn payment(&mut self, id: String) -> Promise;
    // fn payment(&self, j: Job) -> Promise;

    

    
}

// Nhớ là phân insert,

// Implement the contract structure
#[near_bindgen]
impl OutSourcing for Contract {
    // View
    fn client_by_id(&self) -> InfoPerson {
        let owner = env::signer_account_id();

        self.clients.get(&owner).unwrap()
    }
    fn executor_by_id(&self) -> InfoPerson {
        let owner = env::signer_account_id();

        self.exercutors.get(&owner).unwrap()
    }
    fn view_all_executors(&self) -> Vec<InfoPerson> {
        let mut exe = Vec::new();

        for (i, info) in &self.exercutors {
            exe.push(info);
        }

        exe
    }
    fn view_all_clients(&self) -> Vec<InfoPerson> {
        let mut client = Vec::new();

        for (i, info) in &self.clients {
            client.push(info);
        }

        client
    }
    fn view_all_jobs(&self) -> Vec<Job> {
        let mut job = Vec::new();

        for (i, j) in &self.jobs {
            job.push(j);
        }
        job
    }
    fn view_job_by_id(&self, id: String) -> Job {
        if let Some(j) = self.job_by_id.get(&id) {
            j
        } else {
            panic!("This job doesn't exist");
        }
    }

    // Register
    fn register_executor(&mut self, name: String) -> InfoPerson {
        let owner = env::signer_account_id();

        assert!(
            !self.exercutor_by_id.contains_key(&owner),
            "Had an exercutor"
        );
        let executor = InfoPerson {
            name,
            id: owner.clone(),
        };
        self.exercutor_by_id.insert(&owner, &executor);
        self.exercutors.insert(&owner, &executor);

        executor
    }
    fn register_client(&mut self, name: String) -> InfoPerson {
        let owner = env::signer_account_id();

        assert!(!self.client_by_id.contains_key(&owner), "Had an client");

        let client = InfoPerson {
            name,
            id: owner.clone(),
        };
        self.client_by_id.insert(&owner, &client);
        self.clients.insert(&owner, &client);
        
        client
    }

    // Job
    fn create_job(&mut self, id: String, title: String, desc: String, wage: Balance) -> Job {
        let owner = env::signer_account_id();
        assert!(self.client_by_id.contains_key(&owner), "Had an client");
        assert!(!self.job_by_id.contains_key(&id), "Had an job");
        let job = Job {
            id: id.clone(),
            title,
            desc,
            wage,
            owner: owner.clone(),
            check_take: false,
            check_finish: false,
            executor_id: owner,
        };
        self.job_by_id.insert(&id, &job);
        self.jobs.insert(&id, &job);

        job
    }
    fn take_job(&mut self, id: String) -> Job {
        let owner = env::signer_account_id();

        if let Some(mut j) = self.job_by_id.get(&id) {
            if owner.clone() == j.owner {
                panic!("You are client");
            }

            if j.check_take == true {
                panic!("The job has been accepted");
            } else {
                j.check_take = true;
                j.executor_id = owner;
                self.job_by_id.insert(&id, &j);
                self.jobs.insert(&id, &j);
                return j
            }
        } else {
            panic!("There is no {} job", id);
        }
    }
    fn finish_job(&mut self, id: String) -> Job {
        let owner = env::signer_account_id();

        if let Some(mut j) = self.job_by_id.get(&id) {
            if j.executor_id != owner {
                panic!("Not your job");
            } else {
                j.check_finish = true;
                self.job_by_id.insert(&id, &j);
                self.jobs.insert(&id, &j);
                return j
            }
        } else {
            panic!("There is no {} job", id);
        }
    }
    fn update_job_for_client(
        &mut self,
        id: String,
        title: String,
        desc: String,
        wage: Balance,
    ) -> Job {
        let owner = env::signer_account_id();
        let job = Job {
            id: id.clone(),
            title,
            desc,
            wage,
            owner: owner.clone(),
            check_take: false,
            check_finish: false,
            executor_id: owner.clone(),
        };

        if let Some(j) = self.job_by_id.get(&id) {
            assert!(!j.check_take, "You haven't to owner");
            assert_eq!(j.owner, owner, "You haven't to owner");
        } else {
            panic!("There is no {} job", id);
        }

        self.job_by_id.insert(&job.id, &job);
        self.jobs.insert(&job.id, &job);

        job
    }

    // Payment
    #[payable]
    fn payment(&mut self, id: String) -> Promise {
        let owner = env::signer_account_id();
        let job: Job;

        if let Some(j) = self.job_by_id.get(&id) {
            if j.owner != owner && j.wage == env::attached_deposit() / 1000000000000000000000000 {
                panic!("you are not the creator of this job or you don't have enough money");
            } else if j.check_finish == true {
                job = j.clone();
                self.job_by_id.remove(&id);
                self.jobs.remove(&id);
            } else {
                panic!("This work is not finished");
            }
        } else {
            panic!("Don't have that job");
        }

        let job_info = EventLog { job: job.clone() };
        env::log_str(&job_info.to_string());

        Promise::new(job.executor_id).transfer(job.wage)
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            owner_id: env::signer_account_id(),
            clients: UnorderedMap::new(b"clients".try_to_vec().unwrap()),
            client_by_id: LookupMap::new(b"client_by_id".try_to_vec().unwrap()),
            exercutors: UnorderedMap::new(b"exercutors".try_to_vec().unwrap()),
            exercutor_by_id: LookupMap::new(b"exercutor_by.id".try_to_vec().unwrap()),
            jobs: UnorderedMap::new(b"jobs".try_to_vec().unwrap()),
            job_by_id: LookupMap::new(b"job_by_id".try_to_vec().unwrap()),
        }
    }
}
