#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        router,
        types::{AppState, MenuItem},
    };
    use axum::{
        body::Body,
        extract::connect_info::MockConnectInfo,
        http::{self, Request, Response, StatusCode},
        response::IntoResponse,
        routing::delete,
        Router,
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tokio::net::TcpListener;
    use tower::{Service, ServiceExt};

    /// helper function that does a request to the serviceworker to insert `items`` into `table`
    async fn add_items(app: Router, table: usize, items: Vec<usize>) -> Response<Body> {
        app.oneshot(
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

    async fn delete_item(app: Router, table: usize, item: usize) -> Response<Body> {
        app.oneshot(
            Request::builder()
                .method(http::Method::DELETE)
                .uri(format!("/{}/{}/", table, item))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
    }

    /// helper function that does a request to the serviceworker to query items and returns it
    async fn get_items(app: Router, table: usize) -> Result<Vec<MenuItem>, axum::Error> {
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/{}/", table))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .map_err(|e| axum::Error::new(e))?;
        serde_json::from_slice(&response.into_body().collect().await?.to_bytes())
            .map_err(|e| axum::Error::new(e))
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

        let insert_response = add_items(app.clone(), 1, vec![1, 2, 3, 4]).await;

        assert_eq!(insert_response.status(), StatusCode::OK);

        let delete_response = delete_item(app, 1, 1).await;
        assert_eq!(delete_response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn simple_post_test() {
        let app = router();
        let insert_response = add_items(app, 1, vec![1, 2, 3, 4]).await;
        assert_eq!(insert_response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn items_added_exists() -> Result<(), axum::Error> {
        let app = router();
        add_items(app.clone(), 1, vec![1, 2, 3]);
        add_items(app.clone(), 2, vec![4, 5, 6]);
        let menu_items: Vec<u64> = get_items(app, 1)
            .await?
            .iter()
            .map(|i| i.item_number)
            .collect();
        assert_eq!(menu_items, vec![1, 2, 3]);
        Ok(())
    }

    #[tokio::test]
    async fn deletion_works() -> Result<(), axum::Error> {
        let app = router();
        add_items(app.clone(), 1, vec![1, 2, 3]);
        delete_item(app.clone(), 1, 2);
        add_items(app.clone(), 2, vec![4, 5, 6]);
        let menu_items: Vec<u64> = get_items(app.clone(), 1)
            .await?
            .iter()
            .map(|i| i.item_number)
            .collect();
        assert_eq!(menu_items, vec![1, 2]);
        Ok(())
    }

    #[tokio::test]
    async fn deletion_works_by_item_position() -> Result<(), axum::Error> {
        let app = router();
        add_items(app.clone(), 1, vec![10, 20, 30]);
        delete_item(app.clone(), 1, 2);
        add_items(app.clone(), 2, vec![4, 5, 6]);
        let menu_items: Vec<u64> = get_items(app.clone(), 1)
            .await?
            .iter()
            .map(|i| i.item_number)
            .collect();
        assert_eq!(menu_items, vec![10, 30]);
        Ok(())
    }

    #[tokio::test]
    async fn deletion_does_not_disturb_other() -> Result<(), axum::Error> {
        let app = router();
        add_items(app.clone(), 1, vec![10, 20, 30]);
        delete_item(app.clone(), 1, 2);
        add_items(app.clone(), 2, vec![4, 5, 6]);
        let menu_items: Vec<u64> = get_items(app.clone(), 2)
            .await?
            .iter()
            .map(|i| i.item_number)
            .collect();
        assert_eq!(menu_items, vec![4, 5, 6]);
        Ok(())
    }
}
