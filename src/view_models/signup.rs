use crate::models::user::NewUser;
use crate::services::twitter::TwitterUser;
use egg_mode::Token::*;

#[derive(Deserialize)]
pub struct Signup {
    pub twitter_screen_name: String,
    pub twitter_key: String,
    pub twitter_token: String,
}

impl From<TwitterUser> for NewUser {
    fn from(signup: TwitterUser) -> Self {
        let (key, secret) = match signup.token.unwrap() {
            // TODO: Bearer isn't needed.
            Bearer(tk) => (tk, "".to_string()),
            Access { access, consumer } => (access.key.to_string(), access.secret.to_string()),
        };

        NewUser {
            twitter_id: signup.id,
            twitter_key: key,
            twitter_secret: secret,
            twitter_screen_name: signup.screen_name,
        }
    }
}
