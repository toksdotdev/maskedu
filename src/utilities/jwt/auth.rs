use crate::models::user::User;
use crate::utilities::jwt;
use crate::utilities::jwt::UserResult;
use http::StatusCode;
use tower_web::{
    extract::{Context, Error as ExtractError, Extract, Immediate},
    util::BufStream,
    Error,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    user: User,
}

impl Auth {
    pub fn new(user: User) -> Self {
        Self { user }
    }

    pub fn user(&self) -> &User {
        &self.user
    }
}

impl<B: BufStream> Extract<B> for Auth {
    type Future = Immediate<Auth>;

    fn extract(ctx: &Context) -> Self::Future {
        let bearer = match ctx.request().headers().get("Authorization") {
            Some(bearer) => bearer
                .to_str()
                .unwrap_or_default()
                .replace("Bearer", "")
                .trim()
                .to_string(), // could be improved.
            None => {
                return Immediate::err(ExtractError::invalid_argument(&"Authorization is missing."))
            }
        };

        let claims: UserResult = match jwt::decode_token(&bearer) {
            Ok(claim) => claim.into(),
            Err(_) => return Immediate::err(Error::from(StatusCode::UNAUTHORIZED).into()),
        };

        match claims {
            Ok(user) => Immediate::ok(Auth::new(user)),
            Err(_) => return Immediate::err(Error::from(StatusCode::UNAUTHORIZED).into()),
        }
    }
}
