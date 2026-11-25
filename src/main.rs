use axum::{routing::get, Router};
mod calculator;

async fn hello() -> &'static str {
    "Hello, world!"
}

async fn test() -> String {
    let result = calculator::calculate(1, 5, 2).to_string();
    result
}

#[tokio::main]
async fn main() {
let app = Router::new()
    .route("/", get(hello))
    .route("/test", get(test));
    


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}