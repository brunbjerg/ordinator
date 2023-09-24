extern crate chrono;
use chrono::{Utc, TimeZone, DateTime, Duration};


/// The goal of this project is to make a project scheduler that will take a project or goal and 
/// input it into a google calendar
/// 
/// Should we integrate the google calendar as one of the first things? This is a good question.
/// Does it make sense to code on this before that is up and running? Yes I think so. If for nothing
/// else than making it run correctly. This rust project is dividing your attention. This should 
/// only be a side project. Choosing this means no TV
/// 
/// So based on the project and on the scheduling policy we should create structs that represent
/// events. I am in doubt as how to make this work correctly. What type should the .with_ymd_and_hms
/// be called on? This is fundamental to understand. 
/// 
/// TODO
///     TODO : Learn Rust
///     TODO : Make GitHub Repository
///     TODO : Input Goal    
/// 
fn main() {
    println!("Welcome to Ordinator. Personalized Project and Goal Scheduler")
    input_project()
}

fn input_project() {
    println!("Create a project");
    let policy = Policy {
        start: 17,
        end: 20
    };

    let project = Project {
        name: "Create Scheduler",
        policy: policy,
        due: Utc.with_ymd_and_hms(2023, 12, 31, 1, 1, 1),
        hours: 30
    };
}

struct Project {
    name: string,
    policy: Policy,
    due: chrono::DateTime<Utc>,
    hours: i32
}

struct Policy {
    start: i32,
    end: i32
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