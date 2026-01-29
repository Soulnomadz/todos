#[tokio::main]
async fn main() {
    todos::init().await;
    todos::start_server().await;
}

