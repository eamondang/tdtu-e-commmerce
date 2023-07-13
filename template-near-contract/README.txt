. Deploy Code
    npm i -g near-cli
    cargo make prepare 
    cargo make build 
    cargo make dev-deploy

. Register Freelancer
    cargo make call register_executor '{"fullname": "Kuo Nhan Dung", "skills": ["Nodejs","MongoDB"]}' --account-id nkeyskuo124.testnet

. Register Client
    cargo make call register_client '{"organization_name": "SUD Tech", "industry": "Information Technology"}' --account-id nkeyskuo124.testnet

. Create job 
    cargo make call create_job '{"title": "Ticket Buying System", "desc": "Developing a Ticket Buying Platform ...", "budget": 5, "tags": ["ExpressJs","ReactJs","MongoDB"], "duration": 30}' --account-id nkeyskuo124.testnet
    cargo make call create_job '{"title": "Food Ordering System", "desc": "Build an Ordering System for restaurants", "budget": 20, "tags": ["PHP","VueJs","MySQL"], "duration": 20}' --account-id nkeyskuo124.testnet

. Take job
    cargo make call take_job '{"job_id": "J-1689271438776764125"}' --account-id nkeyskuo124.testnet

. View job by id
    cargo make call view_job_by_id '{"job_id": "J-1689271438776764125"}' --account-id nkeyskuo124.testnet

. Update job 
    cargo make call update_job '{"job_id": "J-1689271438776764125", "budget": 4}' --account-id nkeyskuo124.testnet
    cargo make call update_job '{"job_id": "J-1689289254227678863", "duration": 15}' --account-id nkeyskuo124.testnet
. View all jobs
    cargo make call view_all_jobs '{}' --account-id nkeyskuo124.testnet

    Result:
        [
            {
                job_id: 'J-1689271438776764125',
                author: 'nkeyskuo124.testnet',
                executor: null,
                title: 'Ticket Buying System',
                desc: 'Developing a Ticket Buying Platform ...',
                budget: 5,
                tags: [ 'ExpressJs', 'ReactJs', 'MongoDB' ],
                created_at: '1689271438776764125',
                duration: 30,
                status: 'open'
            },
            {
                job_id: 'J-1689289254227678863',
                author: 'nkeyskuo124.testnet',
                executor: null,
                title: 'Food Ordering System',
                desc: 'Build an Ordering System for restaurants',
                budget: 20,
                tags: [ 'PHP', 'VueJs', 'MySQL' ],
                created_at: '1689289254227678863',
                duration: 20,
                status: 'open'
            }
        ]
. View Freelancer 
    cargo make call view_freelancer_by_id --account-id nkeyskuo124.testnet

. Payment 
    cargo make call payment '{"price": 1}' --account-id nkeyskuo124.testnet --amount 1
    cargo make call payment '{"price": 2}' --account-id dev-1689246629823-97048943405890 --amount 1