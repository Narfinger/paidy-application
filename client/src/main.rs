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

#[derive(Debug, Serialize, Deserialize)]
/// A table in the restaurant having various menuitems
pub(crate) struct Table {
    pub(crate) table_number: usize,
    pub(crate) items: Vec<MenuItem>,
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
/// Argument Parsing
struct Args {
    /// Limit the number of results
    #[clap(short, long)]
    limit: Option<usize>,

    #[clap(short, long, value_parser, num_args = 2..,value_delimiter = ' ', group="input", value_names = ["table_number", "menu_item", "menu_item"])]
    /// add menu items to a table, given as `table_number menu_item1 menu_item2...`
    add: Option<Vec<usize>>,

    /// delete a menu item given as `table_number menu_item1 menu_item2...`
    #[clap(short, long, value_parser, num_args = 2..,value_delimiter = ' ', group="input", value_names = ["table_number", "menu_item", "menu_item"])]
    delete: Option<Vec<usize>>,

    /// get all menuitems
    #[clap(long, group = "input")]
    all: bool,

    /// get the current menu items for a table
    #[clap(short = 't', long, group = "input", value_name = "table_number")]
    get_table: Option<usize>,

    /// get specific one
    #[clap(short = 'i', long, num_args = 2, group = "input", value_names = ["table_number", "menu_item"])]
    get_item: Option<Vec<usize>>,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // add
    if let Some(mut add_vec) = args.add {
        let menu_items = add_vec.split_off(1);
        let cl = reqwest::blocking::Client::new();
        cl.post(format!(
            "http://127.0.0.1:3000/{}/?key={}",
            add_vec.first().unwrap(),
            API_KEY
        ))
        .json(&menu_items)
        .send()?;
    // delete
    } else if let Some(mut del_vec) = args.delete {
        let menu_items = del_vec.split_off(1);
        let cl = reqwest::blocking::Client::new();
        for _ in menu_items {
            cl.delete(format!(
                "http://127.0.0.1:3000/{}/?key={}",
                del_vec.first().unwrap(),
                API_KEY
            ))
            .send()?;
        }
    // all
    } else if args.all {
        let query_string = if let Some(l) = args.limit {
            format!("http://127.0.0.1:3000/?key={}&limit={}", API_KEY, l)
        } else {
            format!("http://127.0.0.1:3000/?key={}", API_KEY,)
        };
        let tables = reqwest::blocking::get(query_string)?.json::<Vec<Table>>()?;
        for i in tables {
            println!(
                "--------Showing Items for table {}----------",
                i.table_number
            );
            for (index, menu_item) in i.items.iter().enumerate() {
                println!(
                    "{} | Item#: {} Time: {}",
                    index, menu_item.item_number, menu_item.duration_in_minutes
                );
            }
        }
    // get
    } else if let Some(i) = args.get_table {
        let query_string = if let Some(l) = args.limit {
            format!("http://127.0.0.1:3000/{}/?key={}&limit={}", i, API_KEY, l)
        } else {
            format!("http://127.0.0.1:3000/{}/?key={}", i, API_KEY,)
        };
        let menu_items = reqwest::blocking::get(query_string)?.json::<Vec<MenuItem>>()?;

        println!("--------Showing Items for table {}----------", i);
        for (index, menu_item) in menu_items.iter().enumerate() {
            println!(
                "{} | Item#: {} Time: {}",
                index, menu_item.item_number, menu_item.duration_in_minutes
            );
        }
    } else if let Some(mut i) = args.get_item {
        let menu_item = i.split_off(1);
        let menu_item_number = menu_item.first().unwrap();
        let table = i.first().unwrap();

        let items = reqwest::blocking::get(format!(
            "http://127.0.0.1:3000/{}/{}/?key={}",
            table, menu_item_number, API_KEY,
        ))?
        .json::<Vec<MenuItem>>()?;
        if let Some(item) = items.first() {
            println!(
                "| Item#: {} Time: {}",
                item.item_number, item.duration_in_minutes
            );
        }
    }

    Ok(())
}
