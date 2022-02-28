mod api;
mod config;
mod printer;

use std::ops::Add;
use clap::{Parser, Subcommand};
use crate::api::{Menu, MenuAPI};
use chrono::{Duration, NaiveDate, Utc};
use anyhow::{Context, Result};
use crate::Commands::{Date, Next, Search, Today, Tomorrow, Upcoming};
use crate::config::ConfigHandler;
use crate::printer::MenuPrinter;


/// A CLI to quickly check what is served in your local Mensa.
#[derive(Parser)]
#[clap(version, about)]
struct Arguments {

    /// Show the UUIDs of the menus
    #[clap(short, long)]
    ids: bool,

    /// Override the api remote set in the config
    #[clap(short, long)]
    api: Option<String>,

    /// Which menus to print
    #[clap(subcommand)]
    what: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Print today's menus
    Today {},
    /// Print tomorrow's menus
    Tomorrow {},
    /// Print the menus in x days
    Next {
        /// Days in the future to print menu
        offset: u32
    },
    /// Print the menus on a specific date
    Date {
        /// Date to print menu of (dd.MM.yy)
        date: String
    },
    /// Search for menus to print with a specific query
    Search {
        /// Query to search for
        query: String
    },
    /// Print all menus that are yet to come
    Upcoming { }
}


fn main() -> Result<()>{

    // Load and check config
    let mut config = ConfigHandler::load()?;
    config.check()?;

    // Load arguments
    let args: Arguments = Arguments::parse();

    // Instantiate api and printer
    let api = MenuAPI::new(args.api.map_or_else(|| config.config.api_remote.clone(), |custom| custom));
    let printer = MenuPrinter::new(&config.config, args.ids, &if config.config.display_links { Some(config.config.website_remote.clone()) } else { None });

    // Fetch menus
    let menus = match args.what {
        Today {} => {
            api.read_todays_menus()
        },
        Tomorrow {} => {
            let mut date = Utc::now().naive_local().date();
            date = date.add(Duration::days(1));
            api.read_dated_menus(date)
        },
        Next {offset} => {
            let mut date = Utc::now().naive_local().date();
            date = date.add(Duration::days(offset as i64));
            api.read_dated_menus(date)
        },
        Date {date} => {
            api.read_dated_menus(NaiveDate::parse_from_str(&date, "%d.%m.%y").with_context(|| "Please provide a valid date!")?)
        },
        Search { query } => {
            api.read_menus_search(&query)
        },
        Upcoming {} => {
            api.read_upcoming_menus()
        }
    }.with_context(|| "Couldn't read menus from api")?;

    // Print menus
    println!();
    printer.print_menus(menus);

    Ok(())
}
