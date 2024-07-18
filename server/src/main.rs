use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get},
    Json, Router,
};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use types::{new_app_state, AppState, MenuItem, QueryParam, AMOUNT_OF_TABLES, API_KEY};

mod tests;
mod types;

/// Returns all items for all tables, if supplied the limit applies to the number of tables, not the number of menuitems
/// We do not return tables that do not have menuitems
async fn get_all_items(
    Query(query): Query<QueryParam>,
    State(state): State<AppState>,
) -> Result<String, StatusCode> {
    if query.key != API_KEY {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        let mut all_tables_vector = Vec::new();
        for i in state
            .iter()
            .take(query.limit.unwrap_or(AMOUNT_OF_TABLES as u64) as usize)
        {
            let locked = i.read().await;
            if !locked.items.is_empty() {
                let s = locked.items.to_vec();
                all_tables_vector.push(s);
            }
        }
        serde_json::to_string(&all_tables_vector).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}

/// returns the items for a given `table_id`, table_id start at zero.
async fn get_items_for_table(
    Path(table_number): Path<usize>,
    Query(query): Query<QueryParam>,
    State(state): State<AppState>,
) -> Result<String, StatusCode> {
    if query.key != API_KEY {
        Err(StatusCode::UNAUTHORIZED)
    } else if let Some(table_lock) = state.get(table_number).map(|table| table.read()) {
        let table_lock = table_lock.await;
        let limit = query.limit.unwrap_or(table_lock.items.len() as u64);
        let new_items = table_lock
            .items
            .iter()
            .take(limit as usize)
            .collect::<Vec<&MenuItem>>();
        serde_json::to_string(&new_items).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// returns a specific item
async fn get_item(
    Path((table_number, item_number)): Path<(usize, usize)>,
    Query(query): Query<QueryParam>,
    State(state): State<AppState>,
) -> Result<String, StatusCode> {
    if query.key != API_KEY {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        let json_string = if let Some(table_lock) = state.get(table_number) {
            let table_items = &table_lock.read().await.items;
            if let Some(item) = table_items.get(item_number) {
                serde_json::to_string::<Vec<&MenuItem>>(&vec![item])
            } else {
                serde_json::to_string::<Vec<MenuItem>>(&vec![])
            }
        } else {
            serde_json::to_string::<Vec<MenuItem>>(&vec![])
        }
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(json_string)
    }
}

/// adds items to a table given by `table_id` (starting at zero) with the body a json. Returns if we successfully added the items.
/// Notice that this does not add items to the table if we are out of tables.
async fn add_item_to_table(
    Path(table_number): Path<usize>,
    Query(query): Query<QueryParam>,
    State(state): State<AppState>,
    Json(vec_items): Json<Vec<u64>>,
) -> Result<Json<bool>, StatusCode> {
    if query.key != API_KEY {
        Err(StatusCode::UNAUTHORIZED)
    } else if let Some(table) = state.get(table_number) {
        let mut table_mut = table.write().await;
        for i in vec_items {
            table_mut.items.push(MenuItem::new(i));
        }
        Ok(Json(true))
    } else {
        Ok(Json(false))
    }
}

/// deletes an item from a given `table_id` (starting at zero) and a given `item_position``. Returns if we successfully deleted the item.
async fn delete_item(
    Path((table_number, item_position)): Path<(usize, usize)>,
    Query(query): Query<QueryParam>,
    State(state): State<AppState>,
) -> Result<Json<bool>, StatusCode> {
    if query.key != API_KEY {
        Err(StatusCode::UNAUTHORIZED)
    } else if let Some(table) = state.get(table_number) {
        let mut table_mut = table.write().await;
        table_mut.items.remove(item_position);
        Ok(Json(true))
    } else {
        Ok(Json(false))
    }
}

/// Setup the router with the app state
fn router() -> Router {
    let state: AppState = new_app_state();

    Router::new()
        .route("/", get(get_all_items))
        .route(
            "/:table_number/",
            get(get_items_for_table).post(add_item_to_table),
        )
        .route(
            "/:table_number/:item_number/",
            delete(delete_item).get(get_item),
        )
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
