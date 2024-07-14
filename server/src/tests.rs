#[cfg(test)]
mod tests {
    use super::*;
    use crate::{router, types::AppState};
    use axum::{
        body::Body,
        extract::connect_info::MockConnectInfo,
        http::{self, Request, Response, StatusCode},
        response::IntoResponse,
        Router,
    };
    use serde_json::{json, Value};
    use tokio::net::TcpListener;
    use tower::{Service, ServiceExt};

    /// helper function that does a request to the serviceworker to insert `items`` into `table`
    async fn add_items(app: &Router, table: usize, items: Vec<usize>) -> Response<Body> {
        app.clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri(format!("/{}/", table))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_vec(&json!(items)).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn simple_insert_test() {
        let app = router();
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/1/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn simple_delete_test() {
        let app = router();

        let insert_response = add_items(&app, 1, vec![1, 2, 3, 4]).await;

        assert_eq!(insert_response.status(), StatusCode::OK);

        let delete_response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri("/1/1/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(delete_response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn simple_post_test() {
        let app = router();
        let insert_response = add_items(&app, 1, vec![1, 2, 3, 4]).await;
        assert_eq!(insert_response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn items_added_exists() {}
}
