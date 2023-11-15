use dotenvy::dotenv;
use dotenvy_macro::dotenv;

use data::run;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_creds = dotenv!("DATABASE_URL");
    run(db_creds.to_string()).await
}
