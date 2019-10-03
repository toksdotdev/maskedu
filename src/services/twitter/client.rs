use crate::services::twitter::user::TwitterUser;
use crate::tower_web::codegen::futures::Future;
use egg_mode::Token;
use egg_mode::{access_token, authenticate_url, request_token, tweet::DraftTweet, KeyPair};
use tokio::runtime::current_thread::block_on_all;

#[derive(Debug)]
pub struct Client {
    consumer_token: KeyPair,
    callback_url: String,
}

impl Client {
    pub fn new(key: String, secret: String, callback_url: Option<String>) -> Self {
        Self {
            consumer_token: KeyPair::new(key, secret),
            callback_url: callback_url.unwrap_or("oob".to_string()),
        }
    }

    pub fn consumer_token(&self) -> &KeyPair {
        &self.consumer_token
    }

    // TODO: Handle for failure
    fn request_token(&self) -> KeyPair {
        block_on_all(request_token(&self.consumer_token, &self.callback_url)).unwrap()
    }

    pub fn authenticate(&self) -> String {
        authenticate_url(&self.request_token())
    }

    // TODO: handle for error, when user hasn't authorized with our app.
    pub fn user(&self, verifier: &str) -> TwitterUser {
        let token = access_token(self.consumer_token.clone(), &self.request_token(), verifier);
        let (token, user_id, _) = block_on_all(token).unwrap();
        // In case of rate limiting, see https://docs.rs/egg-mode/0.13.0/egg_mode/struct.Response.html
        let user = block_on_all(egg_mode::user::show(user_id, &token))
            .unwrap()
            .response;

        TwitterUser::with_token(token, user)
    }

    pub fn tweet(&self, message: &str, token: &Token) -> impl Future {
        // This assumes every user struct return has a valid token
        // once error handling is setup on `self.user(...)`, this would set
        // gounds to make this function safe to call.
        DraftTweet::new(message).send(token)
    }
}
