mod homepage;
use axum::{Router, routing::get};
use homepage::homepage;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(homepage));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
