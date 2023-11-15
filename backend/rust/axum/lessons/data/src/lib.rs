use sea_orm::Database;

pub async fn run(database_url: String) {
    let database = Database::connect(database_url).await;
}
