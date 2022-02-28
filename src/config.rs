use std::io::stdin;
use anyhow::{Context, Result};
use reqwest::Url;
use serde::{Serialize, Deserialize};
use crate::MenuAPI;
use crate::printer::FormatConfig;


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_remote: String,
    pub website_remote: String,
    pub display_links: bool,
    pub format: FormatConfig
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            api_remote: "".into(),
            website_remote: "".into(),
            display_links: false,
            format: FormatConfig::default()
        }
    }
}

pub struct ConfigHandler {
    pub config: Config,
}

impl ConfigHandler {
    pub fn load() -> Result<Self> {
        Ok(Self {
            config: confy::load("menu-cli").with_context(|| "Failed to read config")?
        })
    }

    pub fn save(&self) -> Result<()>{
        confy::store("menu-cli", &self.config).with_context(|| "Failed to save config")
    }

    pub fn check(&mut self) -> Result<()>{
        if self.config.website_remote.is_empty() && self.config.api_remote.is_empty() {
            println!("Welcome! It appears that you are using this cli the first time.\nSo please enter the following details in order to use it (You can always change them later in your config file):\n");

            // Enter website remote
            println!("Please enter the website of your menu service. [Example: https://menu.example.com]");
            let mut buf = String::new();
            loop {
                stdin().read_line(&mut buf).with_context(|| "Failed to read string from console input")?;

                if Url::parse(buf.as_str()).map_or_else(|_e| false, |_u| true) {
                    self.config.website_remote = buf.trim().into();
                    break;
                } else {
                    println!("Please enter a valid url.");
                    buf.clear();
                }
            }

            // Enter api remote
            println!("Now enter the API of your menu service. If you don't know its url, try looking for it on the website (presumably in the footer). [Example: https://api.example.com]");
            let mut buf = String::new();
            loop {
                stdin().read_line(&mut buf).with_context(|| "Failed to read string from console input")?;

                if Url::parse(buf.as_str()).map_or_else(|_e| false, |_u| true) {

                    // Check whether api is reachable
                    let api = MenuAPI::new(buf.trim().into());
                    if !api.read_api_info().map_or_else(|_e| false, |_u| true) {
                        println!("Couldn't reach a valid api. Please enter a proper api url.");
                        buf.clear();
                        continue;
                    }

                    self.config.api_remote = buf.trim().into();
                    break;
                } else {
                    println!("Please enter a valid url.");
                    buf.clear();
                }
            }

            self.save()?;
            println!("Configured menu-cli successfully!\n")
        }

        Ok(())
    }
}