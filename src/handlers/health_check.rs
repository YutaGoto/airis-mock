use axum::http::StatusCode;
use axum::response::Response;

pub async fn health_check() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body("OK".into())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use axum::serve;
    use reqwest::Client;
    use reqwest::StatusCode;

    use crate::app;

    #[tokio::test]
    async fn test_health_check() {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap(); // random port
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            serve(listener, app()).await.unwrap();
        });

        let client = Client::new();
        let response = client
            .get(format!("http://{}/health-check", addr))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.text().await.unwrap();
        assert_eq!(body, "OK");
    }
}
