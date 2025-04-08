mod index;
mod upload;
use index::index;
use sqlx::postgres::PgPoolOptions;
use upload::upload;

use axum::{
    Extension, Router,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://ghis:password@10.153.115.29:5432/filesDB")
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(index).post(upload))
        .route("/http://localhost:8000/upload", post(upload))
        .layer(Extension(pool.clone())); // Use the index route for GET and upload route for POST

    let listener = tokio::net::TcpListener::bind("localhost:8000")
        .await
        .expect("Failed to start listener");

    let addr = listener.local_addr().expect("Failed to get local address");

    println!("Application running at http://{}", addr);

    axum::serve(listener, app)
        .await
        .expect("Failed to serve 'app'!");
}
