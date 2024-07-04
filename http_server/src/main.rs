use std::collections::HashMap;

use axum::extract::Path;
use axum::extract::Query;
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/book/:id", get(path_extract))
        .route("/book", get(query_extract));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on 127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>hello world</h1>")
}

async fn path_extract(Path(id): Path<u32>) -> Html<String> {
    Html(format!("hello, {id}!"))
}

async fn query_extract(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    Html(format!("{params:#?}"))
}
