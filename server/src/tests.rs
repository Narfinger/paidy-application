#[cfg(test)]
mod tests {
    use crate::{
        router,
        types::{MenuItem, API_KEY},
    };
    use axum_test::{TestResponse, TestServer};

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

    /// helper function that starts the testserver
    async fn setup_server() -> Result<TestServer, anyhow::Error> {
        TestServer::new(router())
    }

    #[tokio::test]
    /// testing if we can get simple get requests
    async fn simple_insert_test() {
        let server = setup_server().await.unwrap();
        let response = server.get("/1/").add_query_param("key", API_KEY).await;
        response.assert_status_ok();
    }

    #[tokio::test]
    /// testing if we can get simple get requests
    async fn too_large_table() {
        let server = setup_server().await.unwrap();
        let response = server.get("/300/").add_query_param("key", API_KEY).await;

        response.assert_status_not_found();
    }

    #[tokio::test]
    /// testing simple delete requests
    async fn simple_delete_test() {
        let server = setup_server().await.unwrap();

        let insert_response = add_items(&server, 1, vec![1, 2, 3, 4]).await;
        insert_response.assert_status_ok();

        let delete_response = delete_item(&server, 1, 1).await;
        delete_response.assert_status_ok();
    }

    #[tokio::test]
    /// testing simple post requests
    async fn simple_post_test() {
        let server = setup_server().await.unwrap();
        let insert_response = add_items(&server, 1, vec![1, 2, 3, 4]).await;
        insert_response.assert_status_ok();
    }

    #[tokio::test]
    /// testing if items we add via the api will exist when queried
    async fn items_added_exists() {
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
    }

    #[tokio::test]
    /// testing if inserted items via the api can get deleted
    async fn deletion_works() {
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
    }

    #[tokio::test]
    /// making sure that we delete via the item position and not the name of the item
    async fn deletion_works_by_item_position() {
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
    }

    #[tokio::test]
    /// testing that deletion does not delete items on other tables
    async fn deletion_does_not_disturb_other() {
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
    }

    #[tokio::test]
    /// can we get a specific item from a specific table
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
    /// test if we reject with no key parameter supplied
    async fn unauthorized_no_query_param() {
        let server = setup_server().await.unwrap();
        let get = server.get("/1/").await;

        assert_eq!(get.status_code(), crate::StatusCode::BAD_REQUEST);
        let insert = server.post("/1/").json(&vec![1, 2, 3]).await;
        assert_eq!(insert.status_code(), crate::StatusCode::BAD_REQUEST);
        let delete = server.delete("/1/1/").await;
        assert_eq!(delete.status_code(), crate::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    /// test if we reject with the wrong key parameter supplied
    async fn unauthorized_wrong_key() {
        let server = setup_server().await.unwrap();
        //panic!("NYI");
        let get = server.get("/1/").add_query_param("key", "foo").await;
        assert_eq!(get.status_code(), crate::StatusCode::UNAUTHORIZED);
        let insert = server
            .post("/1/")
            .add_query_param("key", "foo")
            .json(&vec![1, 2, 3])
            .await;
        insert.assert_status_unauthorized();
        let delete = server.delete("/1/1/").add_query_param("key", "foo").await;
        assert_eq!(delete.status_code(), crate::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    /// test if we can limit the table status
    async fn limit() {
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

    #[tokio::test]
    /// test if we can limit is too large
    async fn limit_to_large() {
        let server = setup_server().await.unwrap();
        let vec = vec![1, 2, 3, 4];
        let insert = add_items(&server, 1, vec).await;
        insert.assert_status_ok();
        let result = server
            .get("/1/")
            .add_query_param("key", API_KEY)
            .add_query_param("limit", "600")
            .await;
        result.assert_status_ok();
        assert_eq!(
            result
                .json::<Vec<MenuItem>>()
                .iter()
                .map(|m| m.item_number)
                .collect::<Vec<u64>>(),
            vec![1, 2, 3, 4]
        );
    }

    #[tokio::test]
    /// test if we can limit is zero
    async fn limit_is_zero() {
        let server = setup_server().await.unwrap();
        let vec = vec![20; 500];
        let insert = add_items(&server, 1, vec).await;
        insert.assert_status_ok();
        let result = server
            .get("/1/")
            .add_query_param("key", API_KEY)
            .add_query_param("limit", "0")
            .await;
        result.assert_status_ok();
    }

    #[tokio::test]
    /// test if limit is negative
    async fn limit_is_negative() {
        let server = setup_server().await.unwrap();
        let vec = vec![20; 500];
        let insert = add_items(&server, 1, vec).await;
        insert.assert_status_ok();
        let result = server
            .get("/1/")
            .add_query_param("key", API_KEY)
            .add_query_param("limit", "-1")
            .await;
        result.assert_status_bad_request();
    }

    #[tokio::test]
    /// test to get all items
    async fn all_items() {
        let server = setup_server().await.unwrap();
        let insert1 = add_items(&server, 1, vec![10, 20, 30]).await;
        insert1.assert_status_ok();
        let insert2 = add_items(&server, 2, vec![12, 22, 32]).await;
        insert2.assert_status_ok();
        let all_items = server.get("/").add_query_param("key", API_KEY).await;
        all_items.assert_status_ok();

        let item_numbers = all_items
            .json::<Vec<Vec<MenuItem>>>()
            .iter()
            .flatten()
            .map(|mi: &MenuItem| mi.item_number)
            .collect::<Vec<u64>>();

        assert_eq!(item_numbers, vec![10, 20, 30, 12, 22, 32]);
    }

    #[tokio::test]
    /// test if we do not returns tables without MenuItems on them
    async fn all_items_with_gap() {
        let server = setup_server().await.unwrap();
        let insert1 = add_items(&server, 1, vec![10, 20, 30]).await;
        insert1.assert_status_ok();
        let insert2 = add_items(&server, 3, vec![13, 23, 33]).await;
        insert2.assert_status_ok();
        let all_items = server.get("/").add_query_param("key", API_KEY).await;
        all_items.assert_status_ok();

        let item_numbers = all_items
            .json::<Vec<Vec<MenuItem>>>()
            .iter()
            .map(|table| table.iter().map(|mi| mi.item_number).collect())
            .collect::<Vec<Vec<u64>>>();

        assert_eq!(item_numbers, vec![vec![10, 20, 30], vec![13, 23, 33]]);
    }

    #[tokio::test]
    /// test to get all items with a limit
    async fn all_items_limit() {
        let server = setup_server().await.unwrap();
        let insert1 = add_items(&server, 1, vec![10, 20, 30]).await;
        insert1.assert_status_ok();
        let insert2 = add_items(&server, 2, vec![12, 22, 32]).await;
        insert2.assert_status_ok();
        let all_items = server
            .get("/")
            .add_query_param("key", API_KEY)
            .add_query_param("limit", 2)
            .await;
        all_items.assert_status_ok();

        let item_numbers = all_items
            .json::<Vec<Vec<MenuItem>>>()
            .iter()
            .flatten()
            .map(|mi: &MenuItem| mi.item_number)
            .collect::<Vec<u64>>();

        assert_eq!(item_numbers, vec![10, 20, 30]);
    }
}
