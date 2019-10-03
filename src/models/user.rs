use crate::diesel::Connection;
use crate::models::ModelError;
use crate::schema::users;
use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::insert_into;
use diesel::prelude::MysqlConnection;
use diesel::result::Error as DieselError;
use egg_mode::KeyPair;
use std::borrow::Cow;

#[table_name = "users"]
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    #[serde(skip)]
    pub id: i32,
    pub twitter_id: u64,
    pub twitter_screen_name: String,
    pub twitter_key: String,
    pub twitter_secret: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub twitter_id: u64,
    pub twitter_key: String,
    pub twitter_secret: String,
    pub twitter_screen_name: String,
}

impl User {
    pub fn create(user: NewUser, connection: &MysqlConnection) -> Result<User, ModelError> {
        use crate::diesel::ExpressionMethods;
        use crate::diesel::QueryDsl;
        use crate::schema::users::dsl::id;
        use diesel::result::Error;
        use diesel::RunQueryDsl;

        let inserted_user = connection.transaction::<_, Error, _>(|| {
            insert_into(users::table).values(user).execute(connection)?;

            Ok(users::table.order(id.desc()).first::<User>(connection)?)
        });

        Ok(inserted_user?)
    }

    pub fn hash_password(plain: String) -> Result<String, ModelError> {
        Ok(hash(plain, DEFAULT_COST)?)
    }

    pub fn find(id: i32, connection: &MysqlConnection) -> Result<User, ModelError> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        Ok(users::table.find(id).first(connection)?)
    }

    pub fn find_by_twitter_id(id: u64, connection: &MysqlConnection) -> Result<User, ModelError> {
        use crate::schema::users::dsl::twitter_id;
        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let mut records = users::table
            .filter(twitter_id.eq(id))
            .load::<User>(connection)?;

        Ok(records
            .pop()
            .ok_or(ModelError::DatabaseError(DieselError::NotFound))?)
    }

    pub fn twitter_access_token(&self) -> KeyPair {
        // TODO: Improve this implementation.
        KeyPair {
            key: Cow::Owned(self.twitter_key.clone()),
            secret: Cow::Owned(self.twitter_secret.clone()),
        }
    }
}
