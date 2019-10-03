use crate::{utilities::SCHEDULER, TWITTER};
use chrono::{DateTime, Utc};
use clokwerk::TimeUnits;
use egg_mode::{KeyPair, Token};
use tokio::runtime::current_thread::block_on_all;
use tower_web::codegen::futures::Future;

#[derive(Debug)]
pub struct TweetScheduler;

impl TweetScheduler {
    pub fn schedule(message: String, datetime: DateTime<Utc>, user_token: KeyPair) {
        let mut scheduler = SCHEDULER.lock().unwrap();
        let token = Token::Access {
            consumer: TWITTER.consumer_token().clone(),
            access: user_token,
        };

        // TODO: Fix this implementation for every.
        scheduler.every(5.minutes()).run(move || {
            block_on_all(TWITTER.tweet(&message, &token).and_then(|a| {
                // on success, don't re-schedule.
                Ok(a)
            }));
        });
    }
}
