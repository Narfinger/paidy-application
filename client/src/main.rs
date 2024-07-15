use clap::Parser;
use serde::{Deserialize, Serialize};

pub(crate) static API_KEY: &str = "QXlj";

#[derive(Debug, Serialize, Deserialize)]
/// an item on the menu
pub(crate) struct MenuItem {
    /// the number of the menu, i.e., 1 for Potato Fries, 2 for Karaage, etc.
    pub(crate) item_number: u64,
    /// the duration the menu item needs to cook in minutes. We do not need finer granularity.
    pub(crate) duration_in_minutes: u64,
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, value_parser, num_args = 1..,value_delimiter = ' ')]
    add: Option<Vec<u64>>,
    #[clap(short, long)]
    get: Option<usize>,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Some(i) = args.get {
        let menu_items =
            reqwest::blocking::get(format!("http://127.0.0.1:3000/{}/?key={}", i, API_KEY))?
                .json::<Vec<MenuItem>>()?;
        println!("--------Showing Items for table {}----------", i);
        for i in menu_items {
            println!("Item#: {} Time: {}", i.item_number, i.duration_in_minutes);
        }
    } else if let Some(mut add_vec) = args.add {
        let menu_items = add_vec.split_off(1);
        let cl = reqwest::blocking::Client::new();
        cl.post(format!(
            "http://127.0.0.1:3000/{}/?key={}",
            add_vec.first().unwrap(),
            API_KEY
        ))
        .json(&menu_items)
        .send()?;
    }

    Ok(())
}
