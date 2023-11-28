use crate::server;

pub async fn server() {
    server::run().await;
}
