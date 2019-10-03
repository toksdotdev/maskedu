#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tower_web;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate serde;

mod configs;
mod models;
mod resources;
mod schema;
mod services;
mod utilities;
mod view_models;
use crate::configs::twitter::*;
use crate::resources::TweetResource;
use crate::services::twitter::Twitter;
use tower_web::ServiceBuilder;

lazy_static! {
    pub static ref TWITTER: Twitter = {
        Twitter::new(
            TWITTER_CONSUMER_TOKEN.to_string(),
            TWITTER_CONSUMER_SECRET.to_string(),
            Some(TWITTER_CALLBACK_URL.to_string()),
        )
    };
}

fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(TweetResource)
        .run(&addr)
        .unwrap();
}
