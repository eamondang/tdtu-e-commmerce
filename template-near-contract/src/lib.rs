use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};

pub type JobId = u128;
pub type ClientId = AccountId;
pub type ApplicantId = AccountId;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,

    // Job
    pub job_by_id: LookupMap<JobId, Job>,
    pub all_jobs: UnorderedMap<u128, Job>,
    pub total_jobs: u64,

    // Client
    pub client_by_id: LookupMap<ClientId, Client>,
    pub all_clients: UnorderedMap<u128, Client>,
    pub total_clients: u64,

    // Applicant
    pub applicant_by_id: LookupMap<ApplicantId, Applicant>,
    pub all_applicants: UnorderedMap<u128, Applicant>,
    pub total_applicants: u64,

    pub jobs_per_client: UnorderedMap<ClientId, Vec<Job>>,

    pub jobs_apply_by_applicants: UnorderedMap<ApplicantId, Job>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
    id: JobId,
    min_salary: u64,
    max_salary: u64,
    number_job_vacancies: i8,
    company_address: String,
    jd: String,
    client_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Client {
    id: ClientId,
    name: String,
    company: String,
    phone: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Applicant {
    id: ApplicantId,
    name: String,
    cv: String,
    phone: String,
}

pub trait OutSourcing {
    // Đăng ký làm người xin việc
    fn register_applicant(&mut self, name: String, cv: String, phone: String);

    // Đăng ký để làm người tạo job
    fn register_client(&mut self, name: String, company: String, phone: String);

    fn view_all_clients(&self) -> Vec<Client>;

    fn create_job(
        &mut self,
        min_salary: u64,
        max_salary: u64,
        number_job_vacancies: i8,
        company_address: String,
        jd: String,
    );

    // Applicant -> Take
    fn take_job(&mut self, id: JobId);

    // Update
    fn update_job(
        &mut self,
        id: JobId,
        min_salary: u64,
        max_salary: u64,
        number_job_vacancies: i8,
        company_address: String,
        jd: String,
    ) -> Job;

    // Payment
    fn payment(price: Balance) -> Promise;

    // View
    fn view_all_jobs(&self) -> Vec<Job>;
    fn view_job_by_id(&self, id: JobId) -> Job;
    fn view_all_applicants(&self) -> Vec<Applicant>;
    fn view_all_jobs_taken(&self) -> Vec<(AccountId, Job)>;
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn init() -> Self {
        Self {
            owner_id: env::signer_account_id(),

            job_by_id: LookupMap::new(b"job by id".try_to_vec().unwrap()),
            all_jobs: UnorderedMap::new(b"all jobs".try_to_vec().unwrap()),
            total_jobs: 0,

            client_by_id: LookupMap::new(b"client by id".try_to_vec().unwrap()),
            all_clients: UnorderedMap::new(b"all clients".try_to_vec().unwrap()),
            total_clients: 0,

            applicant_by_id: LookupMap::new(b"applicant by id".try_to_vec().unwrap()),
            all_applicants: UnorderedMap::new(b"all applicants".try_to_vec().unwrap()),
            total_applicants: 0,

            jobs_per_client: UnorderedMap::new(b"jobs per client".try_to_vec().unwrap()),

            jobs_apply_by_applicants: UnorderedMap::new(
                b"jobs apply by applicants".try_to_vec().unwrap(),
            ),
        }
    }
}

#[near_bindgen]
impl OutSourcing for Contract {
    fn register_applicant(&mut self, name: String, cv: String, phone: String) {
        let applicant_id = env::signer_account_id();
        assert!(
            !self.applicant_by_id.contains_key(&applicant_id),
            "You have registered Applicant account before"
        );
        self.total_applicants += 1;
        let applicant = Applicant {
            id: applicant_id.clone(),
            name,
            cv,
            phone,
        };
        self.applicant_by_id.insert(&applicant_id, &applicant);
        self.all_applicants
            .insert(&(self.total_applicants as u128), &applicant);
    }

    fn view_all_applicants(&self) -> Vec<Applicant> {
        // refactor

        let mut applicants = Vec::new();
        for ele in self.all_applicants.iter() {
            applicants.push(ele.1);
        }
        applicants
    }

    fn register_client(&mut self, name: String, company: String, phone: String) {
        let client_id = env::signer_account_id();
        assert!(
            !self.client_by_id.contains_key(&client_id),
            "You have registered Client account before"
        );
        self.total_clients += 1;
        let client = Client {
            id: client_id.clone(),
            name,
            company,
            phone,
        };
        
        self.client_by_id.insert(&client_id, &client);
        self.all_clients
            .insert(&(self.total_clients as u128), &client);
    }

    fn view_all_clients(&self) -> Vec<Client> {
        let mut clients = Vec::new();
        for ele in self.all_clients.iter() {
            clients.push(ele.1);
        }
        clients
    }

    fn create_job(
        &mut self,
        min_salary: u64,
        max_salary: u64,
        number_job_vacancies: i8,
        company_address: String,
        jd: String,
    ) {
        assert!(
            self.client_by_id.contains_key(&env::signer_account_id()),
            "You need to register Client account before post job hiring"
        );
        self.total_jobs += 1;
        let job_id = self.total_jobs;
        let job = Job {
            id: job_id as u128,
            min_salary,
            max_salary,
            number_job_vacancies,
            company_address,
            jd,
            client_id: env::signer_account_id(),
        };
        let client_id = env::signer_account_id();
        
        let mut jobs_set: Vec<Job> = self
            .jobs_per_client
            .get(&client_id)
            .unwrap_or_else(|| Vec::new());
        jobs_set.push(job.clone());

        self.jobs_per_client.insert(&client_id, &jobs_set);
        self.job_by_id.insert(&(job_id as u128), &job);
        self.all_jobs.insert(&(job_id as u128), &job);
    }
    
    fn take_job(&mut self, id: JobId) {
        let mut job = self.view_job_by_id(id);
        job.number_job_vacancies -= 1;
        
        self.all_jobs.insert(&job.id, &job);
        self.job_by_id.insert(&job.id, &job);

        self.jobs_apply_by_applicants
            .insert(&env::signer_account_id(), &job);
    }

    fn view_all_jobs_taken(&self) -> Vec<(AccountId, Job)> {
        let mut jobs_taken = Vec::new();
        for ele in self.jobs_apply_by_applicants.iter() {
            jobs_taken.push((ele.0, ele.1));
        }
        jobs_taken
    }

    fn update_job(
        &mut self,
        id: JobId,
        min_salary: u64,
        max_salary: u64,
        number_job_vacancies: i8,
        company_address: String,
        jd: String,
    ) -> Job {
        let mut job_found = self.view_job_by_id(id);
        job_found.min_salary = min_salary;
        job_found.max_salary = max_salary;
        job_found.number_job_vacancies = number_job_vacancies;
        job_found.company_address = company_address;
        job_found.jd = jd;
        
        self.job_by_id.insert(&(id as u128), &job_found);
        self.all_jobs.insert(&(id as u128), &job_found);
        
        job_found
    }

    fn view_all_jobs(&self) -> Vec<Job> {
        let mut jobs = Vec::new();
        for ele in self.all_jobs.iter() {
            jobs.push(ele.1);
        }
        jobs
    }

    fn view_job_by_id(&self, id: JobId) -> Job {
        self.all_jobs.get(&id).unwrap()
    }

    fn payment(price: Balance) -> Promise {
        // assert!(price == env::attached_deposit());
        Promise::new("eamondev.testnet".parse().unwrap()).transfer(price)
    }
}
