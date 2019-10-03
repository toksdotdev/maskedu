use crate::configs::database::DATABASE_URL;
use diesel::prelude::*;
use tower_web::{
    extract::{Context, Extract, Immediate},
    util::BufStream,
};

pub struct Database {
    connection: MysqlConnection,
}

impl Database {
    pub fn connection(&self) -> &MysqlConnection {
        &self.connection
    }
}

impl<B: BufStream> Extract<B> for Database {
    type Future = Immediate<Database>;

    fn extract(_: &Context) -> Self::Future {
        let connection = establish_connection();
        Immediate::ok(Database { connection })
    }
}

pub fn establish_connection() -> MysqlConnection {
    let database_url = &DATABASE_URL;

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url.to_string()))
}
