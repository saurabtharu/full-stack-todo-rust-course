use axum::extract::Path;

pub async fn path_variables(Path(id): Path<i32>) -> String {
    id.to_string()
}

pub async fn hardcoded_path() -> String {
    "you got 42".to_string()
}
