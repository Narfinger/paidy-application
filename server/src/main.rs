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

// we validate against this secret key. Not perfect security but better than nothing.
static SECRET_KEY: &str =
    "QXlj0uzlyckcmhVvvRHfSKzXZZE0K/k7+dyQx2k5Le2HwTdpInoh3VtDiLEV4eJLTX3aUcG+7mVO";

async fn get_items(
    Path(table_id): Path<usize>,
    State(state): State<AppState>,
) -> Result<Json<Vec<MenuItem>>, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

async fn add_item_to_table(
    Path(table_id): Path<usize>,
    State(state): State<AppState>,
) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn delete_item(
    Path(table_id): Path<usize>,
    Path(item_id): Path<usize>,
    State(state): State<AppState>,
) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[tokio::main]
async fn main() {
    let state: AppState = new_app_state();

    let app = Router::new()
        .route("/:table_id/", get(get_items))
        .route("/:table_id/", post(add_item_to_table))
        .route("/:table_id/:item_id/", delete(delete_item))
        .with_state(state);

    println!("Listening on port 127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Cannot listen on port 3000");
    axum::serve(listener, app).await.unwrap();
}
