use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Extension, Json, TypedHeader,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
};
use serde::{Deserialize, Serialize};

use crate::database::users::{self, Entity as Users, Model};

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
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {
        username: sea_orm::ActiveValue::Set(request_user.username),
        password: sea_orm::ActiveValue::Set(request_user.password),
        token: sea_orm::ActiveValue::Set(Some("lsjlshdglshrf9sogoashdg".to_owned())),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}

pub async fn login_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let db_user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        // .filter(users::Column::Password.eq(request_user.password))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        let new_token = "121212131212121212".to_owned();
        let mut user = db_user.into_active_model();
        user.token = sea_orm::ActiveValue::set(Some(new_token));

        let saved_user = user
            .save(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // do the login
        Ok(Json(ResponseUser {
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
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
