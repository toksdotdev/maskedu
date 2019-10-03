mod auth;
mod tweets;

pub use self::tweets::TweetResource;

#[derive(Response)]
pub struct BaseResponse<T> {
    message: String,
    data: Option<T>,
}

// TODO: Create a more re-usable error handler.
#[derive(Response)]
pub struct BaseError<T> {
    #[web(status)]
    code: u16,
    message: String,
    data: Option<T>,
}
