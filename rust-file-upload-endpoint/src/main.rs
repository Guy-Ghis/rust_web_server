mod index;
mod upload;
use index::index;
use upload::upload;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index).post(upload));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to start listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve 'app'!");
}
