use axum::{
    routing::get,
    Router
};
use askama::{
    Template
};
use axum::response::Html;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {}

#[derive(Template)]
#[template(path = "quotes.html")]
struct QuotesTemplate {}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/quotes", get(quotes_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home_handler() -> Html<String> {
    Html(HomeTemplate{}.render().unwrap())
}

async fn quotes_handler() -> Html<String> {
    Html(QuotesTemplate{}.render().unwrap())
}