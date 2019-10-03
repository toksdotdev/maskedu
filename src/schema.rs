table! {
    users (id) {
        id -> Integer,
        twitter_id -> Unsigned<Bigint>,
        twitter_screen_name -> Varchar,
        twitter_key -> Varchar,
        twitter_secret -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
