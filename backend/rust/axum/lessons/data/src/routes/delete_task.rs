use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension,
};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
};
use serde::Deserialize;

use crate::database::tasks::{self, Entity as Tasks};

#[derive(Deserialize)]
pub struct QueryParams {
    soft: bool,
}

pub async fn delete_task(
    Path(task_id): Path<i32>,
    // Extension(database): Extension<DatabaseConnection>,
    State(database): State<DatabaseConnection>,
    Query(query_params): Query<QueryParams>,
) -> Result<(), StatusCode> {
    /*
    let task = if let Some(task) = Tasks::find_by_id(task_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        task.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    let _ = Tasks::delete(task)
        .exec(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    */

    // delete task by id
    /*
    Tasks::delete_by_id(task_id)
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    */

    if query_params.soft {
        let mut task = if let Some(task) = Tasks::find_by_id(task_id)
            .one(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            task.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };
        let now = chrono::Utc::now();
        task.deleted_at = Set(Some(now.into()));
        Tasks::update(task)
            .exec(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        // delete many task
        Tasks::delete_many()
            .filter(tasks::Column::Id.eq(task_id))
            .exec(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(())
}
