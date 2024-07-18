use std::sync::Arc;

use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// For clarity we ignore off by one here
pub(crate) static AMOUNT_OF_TABLES: usize = 50;
/// we validate against this secret key. Not perfect security but better than nothing.
pub(crate) static API_KEY: &str = "QXlj";

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
/// an item on the menu
pub(crate) struct MenuItem {
    /// the number of the menu, i.e., 1 for Potato Fries, 2 for Karaage, etc.
    pub(crate) item_number: u64,
    /// the duration the menu item needs to cook in minutes. We do not need finer granularity.
    pub(crate) duration_in_minutes: u64,
}

impl MenuItem {
    /// Create a new menuitem with a random duration
    pub(crate) fn new(item_number: u64) -> Self {
        let mut rng = rand::thread_rng();
        let val = rng.gen_range(5..16);
        Self {
            item_number,
            duration_in_minutes: val,
        }
    }
}

#[derive(Debug)]
/// A table in the restaurant having various menuitems
pub(crate) struct Table {
    pub(crate) items: Vec<MenuItem>,
}

#[derive(Debug, Serialize, Deserialize)]
/// the query parameter, having the API_key and a optional limit
pub(crate) struct QueryParam {
    /// API Key we will check
    pub(crate) key: String,
    /// The limit if we want
    pub(crate) limit: Option<u64>,
}

/// The whole state of the app is a vector of tables.
/// We use RwLock inside as multiple people rarely will add items to the same table
pub(crate) type AppState = Arc<Vec<RwLock<Table>>>;

/// Create a new AppState, filling the table vector with RwLocks
pub(crate) fn new_app_state() -> AppState {
    let mut tables = Vec::with_capacity(AMOUNT_OF_TABLES);
    for _ in 0..AMOUNT_OF_TABLES {
        tables.push(RwLock::new(Table { items: vec![] }));
    }
    Arc::new(tables)
}
