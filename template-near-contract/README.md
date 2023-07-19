## Remove neardev folder
```
cargo make clean
```

## build and deploy
```
cargo make dev-deploy
```

## init contract
```
cargo make call-self init
```

## create new job
```
cargo make call create_job '{"min_salary": 500, "max_salary": 1000, "number_job_vacancies": 10, "company_address": "HCM", "jd": "Blockchain Dev"}' --account-id thanhtung2410.testnet
```
```
cargo make call create_job '{"min_salary": 900, "max_salary": 1800, "number_job_vacancies": 3, "company_address": "HN", "jd": "Solidity"}' --account-id thanhtung2410.testnet
```

## view all jobs
```
cargo make view view_all_jobs
```

## view job by id
```
cargo make view view_job_by_id '{"id": 1}'
```

## update job
```
cargo make call update_job '{"id": 1, "min_salary"
 : 600, "max_salary": 1200, "number_job_vacancies": 15, "company_address": "HN", "jd": "Rust Dev"}' --account-id thanhtung2410.testnet
```

 ## register client
```
cargo make call register_client '{"name": "Nguyen Van A", "company": "FPT", "phone": "0215201592"}' --account-id thanhtung2410.testnet
```

 ## view all clients
```
cargo make view view_all_clients
```

 ## register applicant
```
cargo make call register_applicant '{"name": "Le Thi B", "cv": "link to Le Thi B cv", "phone": "0321951821"}' --account-id thanhtung2410.testnet
```

## view all applicants
```
cargo make view view_all_applicants
```

## take job
```
cargo make call take_job '{"id": 1}' --account-id thanhtung2410.testnet
```

## view all job taken
```
cargo make view view_all_jobs_taken
```

## payment
```
cargo make call payment '{"price": 1, "id": 2}' --account-id thanhtung2410.testnet --amount 1
```