use crate::models::user::User;
use crate::resources::{BaseError, BaseResponse};
use crate::utilities::{jwt, Database};
use crate::TWITTER;

#[derive(Clone, Debug)]
pub struct AuthResource;

#[derive(Serialize, Deserialize, Extract)]
struct RequestToken {
    redirect: String,
}

#[derive(Serialize, Deserialize, Extract)]
struct TwitterToken {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct JwtToken {
    token: String,
}

impl_web! {
    impl AuthResource {
        #[get("/auth/twitter")]
        #[content_type("json")]
        fn auth(&self) -> Result<BaseResponse<RequestToken>, ()> {
            let redirect = TWITTER.authenticate();

            Ok(
                BaseResponse {
                    message: "Perform a redirect.".to_string(),
                    data: Some(RequestToken { redirect })
                }
            )
        }

        #[post("/auth/twitter")]
        #[content_type("json")]
        fn login(&self, body: TwitterToken,db: Database) -> Result<BaseResponse<JwtToken>, BaseError<()>> {
            let user = TWITTER.user(&body.token);

            let user = User::create(user.into(), db.connection()).map_err(|err|
                BaseError { code: 500, message: err.to_string(), data: None }
            )?;

            let token = jwt::create_token(&user).map_err(|_|
                BaseError { code: 500, message: "Error creating json token.".to_string(), data: None}
            )?;

            Ok(BaseResponse {
                message: "Twitter authentication successfull.".to_string(),
                data: Some(JwtToken{token})
            })
        }
    }
}
