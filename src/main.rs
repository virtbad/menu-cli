mod api;
mod config;

use std::io::{Read};
use clap::{Command, Parser, Subcommand};
use crate::api::{Menu, MenuAPI};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use anyhow::{Context, Result};
use serde::de::IntoDeserializer;
use crate::config::ConfigHandler;


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

    // Instantiate api
    let api = MenuAPI::new(args.api.map_or_else(|| config.config.api_remote, |custom| custom));

    Ok(())
}
