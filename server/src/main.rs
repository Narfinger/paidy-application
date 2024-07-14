use axum::{
    debug_handler,
    extract::{FromRequest, Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use types::{new_app_state, AppState, MenuItem};

mod tests;
mod types;

/// returns the item for a given `table_id`.
async fn get_items(
    Path(table_id): Path<usize>,
    State(state): State<AppState>,
) -> Result<Json<Vec<MenuItem>>, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

/// adds items to a table given by `table_id` with the body a json. Returns if we successfully added the items.
async fn add_item_to_table(
    Path(table_id): Path<usize>,
    State(state): State<AppState>,
    Json(vec_items): Json<Vec<usize>>,
) -> Result<Json<bool>, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

/// deletes an item from a given `table_id` and a given `item_position``. Returns if we successfully deleted the item.
async fn delete_item(
    Path(table_id): Path<usize>,
    Path(item_position): Path<usize>,
    State(state): State<AppState>,
) -> Result<Json<bool>, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

/// Setup the router with the app state
fn router() -> Router {
    let state: AppState = new_app_state();

    Router::new()
        .route("/:table_id/", get(get_items).post(add_item_to_table))
        .route("/:table_id/:item_id/", delete(delete_item))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let app = router();
    println!("Listening on port 127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Cannot listen on port 3000");
    axum::serve(listener, app).await.unwrap();
}
