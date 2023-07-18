use std::fmt;
// pub mod lib;
use crate::{InfoPerson, Job};

use near_sdk::{
    serde::{Deserialize, Serialize},
    Balance,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub struct EventLog {
    // pub client: InfoPerson,
    // pub executor: InfoPerson,
    pub job: Job,
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}
