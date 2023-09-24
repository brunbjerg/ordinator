use chrono::{LocalResult, DateTime, Utc};
use crate::policy::Policy;

pub struct Project<'a> {
    name: &'a str,
    policy: Policy,
    due: chrono::LocalResult<DateTime<Utc>>,
    hours: i32
}

impl<'a> Project<'a> {
    pub fn new(name: &'a str, policy: Policy, due: LocalResult<DateTime<Utc>>, hours: i32) -> Self {
        Self { name, policy, due, hours }
    }
}