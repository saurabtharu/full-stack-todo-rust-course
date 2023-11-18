use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 6, message = "must have at least 8 characters"))]
    pub password: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestUser
where
    // these bounds are required by `async_trait`
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(request: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = request
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }
        Ok(user)
    }
}

/*
#[derive(Debug, Serialize)]
pub struct UserResponse {
    sender_request: UserRequest,
    message: String,
}
*/

pub async fn custom_json_extrator(user: RequestUser) {
    dbg!(user);
}
