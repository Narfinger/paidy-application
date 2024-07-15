#[cfg(test)]
mod tests {
    use std::{net::SocketAddr, sync::Arc};

    use super::*;
    use crate::{
        router,
        types::{AppState, MenuItem},
    };
    use axum::http::{self, Request, Response, StatusCode};
    use http_body_util::BodyExt;
    use reqwest::Body;
    use serde_json::{json, Value};
    use tokio::net::TcpListener;

    /// helper function that does a request to the serviceworker to insert `items`` into `table`
    async fn add_items(
        addr: SocketAddr,
        table: usize,
        items: Vec<usize>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let cl = reqwest::Client::new();
        cl.post(format!("http://{addr}/{}/", table))
            .json(&items)
            .send()
            .await
    }

    async fn delete_item(
        addr: SocketAddr,
        table: usize,
        item: usize,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let cl = reqwest::Client::new();
        cl.delete(format!("http://{addr}/{}/{}/", table, item))
            .send()
            .await
    }

    /// helper function that does a request to the serviceworker to query items and returns it
    async fn get_items(addr: SocketAddr, table: usize) -> Result<Vec<MenuItem>, reqwest::Error> {
        let response = reqwest::get(format!("http://{addr}/{}/", table)).await?;
        response.json().await
    }

    async fn setup_server() -> Option<SocketAddr> {
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, router()).await.unwrap();
        });
        Some(addr)
    }

    #[tokio::test]
    async fn simple_insert_test() {
        let addr = setup_server().await.unwrap();
        let response = reqwest::get(format!("http://{addr}/1/",)).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn simple_delete_test() {
        let addr = setup_server().await.unwrap();

        let insert_response = add_items(addr, 1, vec![1, 2, 3, 4]).await.unwrap();

        assert_eq!(insert_response.status(), StatusCode::OK);

        let delete_response = delete_item(addr, 1, 1).await.unwrap();
        assert_eq!(delete_response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn simple_post_test() {
        let addr = setup_server().await.unwrap();
        let insert_response = add_items(addr, 1, vec![1, 2, 3, 4]).await.unwrap();
        assert_eq!(insert_response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn items_added_exists() -> Result<(), reqwest::Error> {
        let addr = setup_server().await.unwrap();
        add_items(addr, 1, vec![1, 2, 3]).await.unwrap();
        add_items(addr, 2, vec![4, 5, 6]).await.unwrap();
        let menu_items: Vec<u64> = get_items(addr, 1)
            .await?
            .iter()
            .map(|i| i.item_number)
            .collect();
        assert_eq!(menu_items, vec![1, 2, 3]);
        Ok(())
    }

    #[tokio::test]
    async fn deletion_works() -> Result<(), reqwest::Error> {
        let addr = setup_server().await.unwrap();
        add_items(addr, 1, vec![1, 2, 3]).await?;
        delete_item(addr, 1, 2).await?;
        add_items(addr, 2, vec![4, 5, 6]).await?;
        let menu_items: Vec<u64> = get_items(addr, 1)
            .await?
            .iter()
            .map(|i| i.item_number)
            .collect();
        assert_eq!(menu_items, vec![1, 2]);
        Ok(())
    }

    #[tokio::test]
    async fn deletion_works_by_item_position() -> Result<(), reqwest::Error> {
        let addr = setup_server().await.unwrap();
        add_items(addr, 1, vec![10, 20, 30]).await?;
        delete_item(addr, 1, 2).await?;
        add_items(addr, 2, vec![4, 5, 6]).await?;
        let menu_items: Vec<u64> = get_items(addr, 1)
            .await?
            .iter()
            .map(|i| i.item_number)
            .collect();
        assert_eq!(menu_items, vec![10, 30]);
        Ok(())
    }

    #[tokio::test]
    async fn deletion_does_not_disturb_other() -> Result<(), reqwest::Error> {
        let addr = setup_server().await.unwrap();
        add_items(addr, 1, vec![10, 20, 30]).await?;
        delete_item(addr, 1, 2).await?;
        add_items(addr, 2, vec![4, 5, 6]).await?;
        let menu_items: Vec<u64> = get_items(addr, 2)
            .await?
            .iter()
            .map(|i| i.item_number)
            .collect();
        assert_eq!(menu_items, vec![4, 5, 6]);
        Ok(())
    }
}
