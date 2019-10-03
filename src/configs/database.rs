lazy_static! {
    pub static ref DATABASE_URL: String = format!(
        "{}://{}:{}@{}/{}",
        env!("DB_CLIENT"),
        env!("DB_USERNAME"),
        env!("DB_PASSWORD"),
        env!("DB_HOST"),
        env!("DB_NAME")
    );
}
