use std::sync::Arc;

use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

static INITIAL_AMOUNT_OF_TABLES: u64 = 50;
// we validate against this secret key. Not perfect security but better than nothing.
static SECRET_KEY: &str =
    "QXlj0uzlyckcmhVvvRHfSKzXZZE0K/k7+dyQx2k5Le2HwTdpInoh3VtDiLEV4eJLTX3aUcG+7mVO";

#[derive(Debug, Serialize, Deserialize)]
/// an item on the menu
pub(crate) struct MenuItem {
    /// the number of the menu, i.e., 1 for Potato Fries, 2 for Karaage, etc.
    pub(crate) item_number: u64,
    /// the duration the menu item needs to cook in minutes. We do not need finer granularity.
    pub(crate) duration_in_minutes: u64,
}

impl MenuItem {
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

/// The whole state of the app is a vector of tables.
/// We use RwLock inside as multiple people rarely will add items to the same table
pub(crate) type AppState = Arc<Vec<RwLock<Table>>>;

pub(crate) fn new_app_state() -> AppState {
    let tables = [1..INITIAL_AMOUNT_OF_TABLES]
        .iter()
        .map(|i| RwLock::new(Table { items: vec![] }))
        .collect::<Vec<RwLock<Table>>>();

    Arc::new(tables)
}
