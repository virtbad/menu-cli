mod api;

use std::io::Read;
use clap::{Command, Parser, Subcommand};
use crate::api::{Menu, MenuAPI};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use anyhow::{Context, Result};


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
    /// Prints today's menus
    Today {},
    /// Prints tomorrow's menus
    Tomorrow {},
    /// Prints the menus in x days
    Next {
        /// Days in the future to print menu
        offset: u32
    },
    /// Prints the menus on a specific date
    Date {
        /// Date to print menu of (dd.MM.yy)
        date: String
    },
    /// Searches for menus to print with a specific query
    Search {
        /// Query to search for
        query: String
    },
    /// Prints all menus that are yet to come
    Upcoming { }
}


fn main() -> Result<()>{

    let args = Arguments::parse();

    return Ok(());
}
