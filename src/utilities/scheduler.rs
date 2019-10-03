use chrono::Utc;
use clokwerk::Scheduler;
use std::sync::Mutex;

lazy_static! {
    pub static ref SCHEDULER: Mutex<Scheduler<chrono::Utc>> = Mutex::new(Scheduler::with_tz(Utc));
}
