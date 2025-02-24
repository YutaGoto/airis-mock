use axum::{
    routing::{get, post},
    Router,
};

mod handlers;
mod models;

use crate::handlers::check_teikyou_unique_search_servlet::check_teikyou_unique_search_servlet;
use crate::handlers::get_body_types::get_body_types;
use crate::handlers::health_check::health_check;
use crate::handlers::root::root;
use crate::handlers::teikyou_unique_search_servlet::teikyou_unique_search_servlet;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/body-types", get(get_body_types))
        .route("/health-check", get(health_check))
        .route("/teikyou/check", post(check_teikyou_unique_search_servlet))
        .route(
            "/teikyou/UniqueSearchServlet",
            post(teikyou_unique_search_servlet),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4567").await.unwrap();

    #[cfg(debug_assertions)]
    println!("Server is running on http://localhost:4567");

    axum::serve(listener, app).await.unwrap();
}
