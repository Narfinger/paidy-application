use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::{self, TraceLayer},
};
use tracing::{info, trace, Level};
use types::{new_app_state, AppState, MenuItem};

mod tests;
mod types;

/// returns the item for a given `table_id`, table_id start at zero.
async fn get_items(
    Path(table_id): Path<usize>,
    State(state): State<AppState>,
) -> Result<String, StatusCode> {
    //panic!("CHECK SECRET");
    let json_string = if let Some(table_lock) = state.get(table_id) {
        let table_items = &table_lock.read().await.items;
        serde_json::to_string(table_items)
    } else {
        serde_json::to_string::<Vec<MenuItem>>(&vec![])
    }
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(json_string)
}

/// adds items to a table given by `table_id` (starting at zero) with the body a json. Returns if we successfully added the items.
/// Notice that this does not add items to the table if we are out of tables.
async fn add_item_to_table(
    Path(table_id): Path<usize>,
    State(state): State<AppState>,
    Json(vec_items): Json<Vec<u64>>,
) -> Result<Json<bool>, StatusCode> {
    if let Some(table) = state.get(table_id) {
        let mut table_mut = table.write().await;
        for i in vec_items {
            table_mut.items.push(MenuItem::new(i));
        }
        Ok(Json(true))
    } else {
        Ok(Json(false))
    }

    //panic!("CHECK SECRET");
}

/// deletes an item from a given `table_id` (starting at zero) and a given `item_position``. Returns if we successfully deleted the item.
async fn delete_item(
    Path((table_id, item_position)): Path<(usize, usize)>,
    State(state): State<AppState>,
) -> Result<Json<bool>, StatusCode> {
    if let Some(table) = state.get(table_id) {
        let mut table_mut = table.write().await;
        table_mut.items.remove(item_position);
        Ok(Json(true))
    } else {
        Ok(Json(false))
    }
    //panic!("CHECK SECRET");
}

/// Setup the router with the app state
fn router() -> Router {
    let state: AppState = new_app_state();

    Router::new()
        .route("/:table_id/", get(get_items).post(add_item_to_table))
        .route("/:table_id/:item_id/", delete(delete_item))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

#[tokio::main]
async fn main() {
    let app = router();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    println!("Listening on port 127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Cannot listen on port 3000");
    axum::serve(listener, app).await.unwrap();
}
