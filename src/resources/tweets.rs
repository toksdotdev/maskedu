use crate::resources::BaseResponse;
use crate::utilities::jwt::Auth;
use crate::utilities::TweetScheduler;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct TweetResource;

#[derive(Serialize, Deserialize, Extract)]
struct Tweet {
    message: String,
    datetime: DateTime<Utc>,
}

impl_web! {
    impl TweetResource {
        #[post("/tweet")]
        #[content_type("json")]
        fn schedule(&self, auth: Auth, tweet:Tweet) -> Result<BaseResponse<()>, ()> {
            let token = auth.user().twitter_access_token();
            TweetScheduler::schedule(tweet.message, tweet.datetime, token);

            Ok(BaseResponse { message: "Tweet scheduled successfully.".to_string(), data: None })
        }
    }
}
