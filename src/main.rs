use std::net::{Ipv4Addr, SocketAddr};

use axum::{
    extract::Multipart,
    routing::{get, on, MethodFilter},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;

const CONTENT_LENGTH_LIMIT: usize = 20 * 1024 * 1024;

#[tokio::main]
async fn main() {
    // configure your cors setting
    let cors_layer = CorsLayer::permissive();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/upload", on(MethodFilter::POST, upload))
        .layer(RequestBodyLimitLayer::new(CONTENT_LENGTH_LIMIT))
        .layer(cors_layer);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
}
