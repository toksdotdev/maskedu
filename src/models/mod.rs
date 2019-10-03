pub mod user;

pub enum ModelError {
    NoDatabaseConnection,
    DatabaseError(diesel::result::Error),
    HashingError(bcrypt::BcryptError),
    NotFound,
}

impl From<diesel::result::Error> for ModelError {
    fn from(error: diesel::result::Error) -> Self {
        ModelError::DatabaseError(error)
    }
}

impl From<bcrypt::BcryptError> for ModelError {
    fn from(error: bcrypt::BcryptError) -> Self {
        ModelError::HashingError(error)
    }
}

impl ToString for ModelError {
    fn to_string(&self) -> String {
        match self {
            Self::NoDatabaseConnection => "No database connection found".to_string(),
            Self::DatabaseError(err) => format!("A database error occured ( {:?} )", err),
            Self::HashingError(err) => "An error occured while hashing".to_string(),
            Self::NotFound => "The specified model requested was not found".to_string(),
        }
    }
}
