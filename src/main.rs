extern crate chrono;
use chrono::{Utc, TimeZone, DateTime};
mod project;
mod policy;

use project::Project;
use policy::Policy;

fn main() {
    println!("Welcome to Ordinator. Personalized Project and Goal Scheduler");
    input_project();
}

fn input_project() {
    let policy = Policy::new(17, 20);
    let project = Project::new("Create Scheduler", policy, Utc.with_ymd_and_hms(2023, 12, 31, 1, 1, 1), 30);
}


// {
//     "summary": "Meeting with Team",
//     "location": "Conference Room",
//     "description": "Monthly team meeting.",
//     "start": {
//       "dateTime": "2023-09-30T10:00:00",
//       "timeZone": "America/Los_Angeles"
//     },
//     "end": {
//       "dateTime": "2023-09-30T11:00:00",
//       "timeZone": "America/Los_Angeles"
//     },
//     "attendees": [
//       {"email": "attendee1@example.com"},
//       {"email": "attendee2@example.com"}
//     ],
//     "visibility": "default"
//   }