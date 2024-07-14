use std::sync::Arc;

use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
/// an item on the menu
pub(crate) struct MenuItem {
    /// the number of the menu, i.e., 1 for Potato Fries, 2 for Karaage, etc.
    pub(crate) item_number: u64,
    /// the duration the menu item needs to cook.
    pub(crate) duration_in_minutes: u64,
}

impl MenuItem {
    fn new(item_number: u64) -> Self {
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
    items: Vec<MenuItem>,
}

pub(crate) type AppState = Arc<Vec<RwLock<Table>>>;

pub(crate) fn new_app_state() -> AppState {
    Arc::new(Vec::new())
}
