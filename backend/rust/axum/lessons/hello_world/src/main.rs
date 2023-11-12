use hello_world;

#[tokio::main]
async fn main() {
    hello_world::run().await;
}
