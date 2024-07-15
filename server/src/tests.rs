#[cfg(test)]
mod tests {
    use std::{net::SocketAddr, sync::Arc};

    use super::*;
    use crate::{
        router,
        types::{AppState, MenuItem, API_KEY},
    };
    use axum::http::{self, Request, Response, StatusCode};
    use axum_test::{TestResponse, TestServer};
    use http_body_util::BodyExt;
    use reqwest::Body;
    use serde_json::{json, Value};
    use tokio::net::TcpListener;

    /// helper function that does a request to the serviceworker to insert `items`` into `table`
    async fn add_items(server: &TestServer, table: usize, items: Vec<usize>) -> TestResponse {
        server
            .post(&format!("/{}/", table))
            .add_query_param("key", API_KEY)
            .json(&items)
            .await
    }

    /// helper function that does a delete request for `table` on `item_position`
    async fn delete_item(server: &TestServer, table: usize, item_position: usize) -> TestResponse {
        server
            .delete(&format!("/{}/{}/", table, item_position))
            .add_query_param("key", API_KEY)
            .await
    }

    /// helper function that does a request to the serviceworker to query items and returns it
    async fn get_items(server: &TestServer, table: usize) -> Vec<MenuItem> {
        server
            .get(&format!("/{}/", table))
            .add_query_param("key", API_KEY)
            .await
            .json()
    }

    async fn setup_server() -> Result<TestServer, anyhow::Error> {
        TestServer::new(router())
    }

    #[tokio::test]
    async fn simple_insert_test() {
        let server = setup_server().await.unwrap();
        let response = server.get("/1/").add_query_param("key", API_KEY).await;
        response.assert_status_ok();
    }

    #[tokio::test]
    async fn simple_delete_test() -> Result<(), reqwest::Error> {
        let server = setup_server().await.unwrap();

        let insert_response = add_items(&server, 1, vec![1, 2, 3, 4]).await;
        insert_response.assert_status_ok();

        let delete_response = delete_item(&server, 1, 1).await;
        delete_response.assert_status_ok();

        Ok(())
    }

    #[tokio::test]
    async fn simple_post_test() {
        let server = setup_server().await.unwrap();
        let insert_response = add_items(&server, 1, vec![1, 2, 3, 4]).await;
        insert_response.assert_status_ok();
    }

    #[tokio::test]
    async fn items_added_exists() -> Result<(), reqwest::Error> {
        let server = setup_server().await.unwrap();
        let insert1 = add_items(&server, 1, vec![1, 2, 3]).await;
        insert1.assert_status_ok();
        assert_eq!(insert1.json::<bool>(), true);
        let insert2 = add_items(&server, 2, vec![4, 5, 6]).await;
        insert2.assert_status_ok();
        assert_eq!(insert2.json::<bool>(), true);
        let menu_items: Vec<u64> = get_items(&server, 1)
            .await
            .iter()
            .map(|i| i.item_number)
            .collect();
        assert_eq!(menu_items, vec![1, 2, 3]);
        Ok(())
    }

    #[tokio::test]
    async fn deletion_works() -> Result<(), reqwest::Error> {
        let server = setup_server().await.unwrap();
        let insert1 = add_items(&server, 1, vec![1, 2, 3]).await;
        insert1.assert_status_ok();
        assert_eq!(insert1.json::<bool>(), true);

        let delete1 = delete_item(&server, 1, 2).await;
        assert_eq!(delete1.json::<bool>(), true);
        delete1.assert_status_ok();

        let insert2 = add_items(&server, 2, vec![4, 5, 6]).await;
        insert2.assert_status_ok();
        assert_eq!(insert2.json::<bool>(), true);
        let menu_items: Vec<u64> = get_items(&server, 1)
            .await
            .iter()
            .map(|i| i.item_number)
            .collect();
        assert_eq!(menu_items, vec![1, 2]);
        Ok(())
    }

    #[tokio::test]
    async fn deletion_works_by_item_position() -> Result<(), reqwest::Error> {
        let server = setup_server().await.unwrap();
        let insert1 = add_items(&server, 1, vec![10, 20, 30]).await;
        insert1.assert_status_ok();
        assert_eq!(insert1.json::<bool>(), true);

        let delete1 = delete_item(&server, 1, 2).await;
        delete1.assert_status_ok();
        assert_eq!(delete1.json::<bool>(), true);

        let insert2 = add_items(&server, 2, vec![4, 5, 6]).await;
        insert2.assert_status_ok();
        assert_eq!(insert2.json::<bool>(), true);
        let menu_items: Vec<u64> = get_items(&server, 1)
            .await
            .iter()
            .map(|i| i.item_number)
            .collect();
        assert_eq!(menu_items, vec![10, 20]);
        Ok(())
    }

    #[tokio::test]
    async fn deletion_does_not_disturb_other() -> Result<(), reqwest::Error> {
        let server = setup_server().await.unwrap();
        let insert1 = add_items(&server, 1, vec![10, 20, 30]).await;
        insert1.assert_status_ok();
        assert_eq!(insert1.json::<bool>(), true);

        let delete1 = delete_item(&server, 1, 2).await;
        delete1.assert_status_ok();
        assert_eq!(delete1.json::<bool>(), true);

        let insert2 = add_items(&server, 2, vec![4, 5, 6]).await;
        insert2.assert_status_ok();
        assert_eq!(insert2.json::<bool>(), true);
        let menu_items: Vec<u64> = get_items(&server, 2)
            .await
            .iter()
            .map(|i| i.item_number)
            .collect();
        assert_eq!(menu_items, vec![4, 5, 6]);
        Ok(())
    }

    #[tokio::test]
    async fn get_specific_item() {
        let server = setup_server().await.unwrap();
        let insert1 = add_items(&server, 1, vec![10, 20, 30]).await;
        insert1.assert_status_ok();
        assert_eq!(insert1.json::<bool>(), true);

        let get = server.get("/1/1/").add_query_param("key", API_KEY).await;
        get.assert_status_ok();
        assert_eq!(get.json::<Vec<MenuItem>>().first().unwrap().item_number, 20);
    }

    #[tokio::test]
    async fn test_unauthorized_no_query_param() {
        let server = setup_server().await.unwrap();
        let get = server.get("/1/").await;

        assert_eq!(get.status_code(), StatusCode::BAD_REQUEST);
        let insert = server.post("/1/").json(&vec![1, 2, 3]).await;
        assert_eq!(insert.status_code(), StatusCode::BAD_REQUEST);
        let delete = server.delete("/1/1/").await;
        assert_eq!(delete.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_unauthorized_wrong_key() {
        let server = setup_server().await.unwrap();
        //panic!("NYI");
        let get = server.get("/1/").add_query_param("key", "foo").await;
        assert_eq!(get.status_code(), StatusCode::UNAUTHORIZED);
        let insert = server
            .post("/1/")
            .add_query_param("key", "foo")
            .json(&vec![1, 2, 3])
            .await;
        insert.assert_status_unauthorized();
        let delete = server.delete("/1/1/").add_query_param("key", "foo").await;
        assert_eq!(delete.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_limit() {
        let server = setup_server().await.unwrap();
        let vec = vec![20; 500];
        let insert = add_items(&server, 1, vec).await;
        insert.assert_status_ok();
        let result = server
            .get("/1/")
            .add_query_param("key", API_KEY)
            .add_query_param("limit", "50")
            .await;
        result.assert_status_ok();
        assert_eq!(result.json::<Vec<MenuItem>>().len(), 50);
    }
}
