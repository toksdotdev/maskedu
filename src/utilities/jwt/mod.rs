mod auth;
mod claims;

pub use self::auth::Auth;
use self::claims::Claims;
use crate::configs::auth::{APP_KEY, APP_NAME};
use crate::models::user::User;
use crate::models::ModelError;
use jsonwebtoken::{decode, encode, errors::Error, Header, Validation};

type UserResult = Result<User, ModelError>;

pub fn create_token(user: &User) -> Result<String, Error> {
    let claims = Claims::new(user.id);
    encode(&Header::default(), &claims, APP_NAME.as_bytes())
}

pub fn decode_token(token: &str) -> Result<Claims, Error> {
    decode::<Claims>(token, APP_KEY.as_bytes(), &Validation::default())
        .map(|data| data.claims.into())
}
