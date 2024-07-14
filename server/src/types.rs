use std::sync::Arc;

use rand::Rng;
use tokio::sync::RwLock;

#[derive(Debug)]
pub(crate) struct MenuItem {
    item: u64,
    duration_in_minutes: u64,
}

impl MenuItem {
    fn new(item: u64) -> Self {
        let mut rng = rand::thread_rng();
        let val = rng.gen_range(5..16);
        Self {
            item,
            duration_in_minutes: val,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Table {
    items: Vec<MenuItem>,
}

pub(crate) type AppState = Arc<RwLock<Vec<Table>>>;

pub(crate) fn new_app_state() -> AppState {
    Arc::new(RwLock::new(Vec::new()))
}
