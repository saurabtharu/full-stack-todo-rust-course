use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Extension, Json, TypedHeader,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::users::{self, Entity as Users, Model},
    utils::{app_error::AppError, jwt::create_jwt},
};

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String,
}

pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, AppError> {
    let jwt = create_jwt()?;
    let new_user = users::ActiveModel {
        username: sea_orm::ActiveValue::Set(request_user.username),
        password: sea_orm::ActiveValue::Set(hash_password(request_user.password)?),
        token: sea_orm::ActiveValue::Set(Some(jwt)),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_err| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"))?;

    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}

pub async fn login_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, AppError> {
    let db_user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        // .filter(users::Column::Password.eq(request_user.password))
        .one(&database)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))?;

    if let Some(db_user) = db_user {
        if !verify_password(request_user.password, &db_user.password)? {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "The password you entered is incorrect",
            ));
        }

        let new_token = create_jwt()?;
        let mut user = db_user.into_active_model();
        user.token = sea_orm::ActiveValue::set(Some(new_token));

        let saved_user = user.save(&database).await.map_err(|_| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
        })?;

        // do the login
        Ok(Json(ResponseUser {
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "User not found."))
    }
}

pub async fn logout(
    // authorization: TypedHeader<Authorization<Bearer>>,
    Extension(database): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();

    user.token = sea_orm::ActiveValue::Set(None);

    user.save(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

fn hash_password(password: String) -> Result<String, AppError> {
    bcrypt::hash(password, 12).map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error. Please try again",
        )
    })
}

fn verify_password(password: String, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash).map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "The password you entered is incorrect",
        )
    })
}
