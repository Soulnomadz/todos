#[tokio::main]
async fn main() {
    todos::init();
    todos::start_server().await;
}

