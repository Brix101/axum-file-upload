use std::net::{Ipv4Addr, SocketAddr};

use axum::{
    extract::{DefaultBodyLimit, Multipart},
    routing::{get, on, post, MethodFilter},
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/upload", on(MethodFilter::POST, upload))
        .layer(DefaultBodyLimit::max(1024));

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        // let name = field.name().unwrap().to_string();
        // let data = field.bytes().await.unwrap();
        //
        // println!("Length of `{}` is {} bytes", name, data.len());
        //
        println!("{:#?}", field);
    }
}
