use crate::configs::auth::*;
use crate::models::user::User;
use crate::models::ModelError;

use crate::utilities::jwt::UserResult;
use chrono::{Duration, Local};
use diesel::prelude::MysqlConnection;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub company: String,
    pub exp: usize,
    #[serde(skip)]
    connection: Option<MysqlConnection>,
}

impl Claims {
    pub fn new(user_id: i32) -> Self {
        Claims {
            sub: user_id,
            company: APP_NAME.to_string(),
            exp: (Local::now() + Duration::hours(24)).timestamp() as usize,
            connection: None, // TODO: handle this properly
        }
    }

    pub fn connection(&self) -> Option<&MysqlConnection> {
        self.connection.as_ref()
    }
}

impl From<Claims> for UserResult {
    fn from(claim: Claims) -> Self {
        let connection = claim.connection().ok_or(ModelError::NoDatabaseConnection)?;
        Ok(User::find(claim.sub, connection)?)
    }
}

impl fmt::Debug for Claims {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
