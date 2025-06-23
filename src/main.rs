use axum::{
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home_handler() -> String {
    "The Home Page".to_string()
}