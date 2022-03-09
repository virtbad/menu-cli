use std::io::Read;
use serde::Deserialize;
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use chrono::serde::ts_milliseconds;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Menu {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(with = "ts_milliseconds")]
    pub date: DateTime<Utc>,
    pub channel: u8,
    pub label: u8,
    pub prices: Vec<Price>,
    #[serde(rename = "voteBalance")]
    pub votes: i32,
}

#[derive(Deserialize)]
pub struct Price {
    pub tag: String,
    pub price: f32,
}

#[derive(Deserialize)]
pub struct ApiInformation {
    pub version: String,
    #[serde(with = "ts_milliseconds")]
    pub started: DateTime<Utc>
}

pub struct MenuAPI {
    url: String,
}

impl MenuAPI {
    pub fn new(mut url: String) -> Self {
        if !url.ends_with('/') {
            url.push('/');
        }

        Self {
            url
        }
    }

    pub fn read_json_string(&self, sub_url: &str) -> Result<String> {
        let mut base = self.url.clone();
        base.push_str(sub_url);

        let mut response: String = String::new();
        reqwest::blocking::get(base.as_str()).with_context(|| format!("Failed to connect to api on '{}'", base))?.read_to_string(&mut response).with_context(|| format!("Failed to read content on '{}'", base))?;

        Ok(response)
    }

    pub fn read_upcoming_menus(&self) -> Result<Vec<Menu>> {
        serde_json::from_str(self.read_json_string("/menu/upcoming")?.as_str()).with_context(|| "Failed to parse json from upcoming menus.")
    }

    pub fn read_todays_menus(&self) -> Result<Vec<Menu>> {
        serde_json::from_str(self.read_json_string("/menu/date")?.as_str()).with_context(|| "Failed to parse json from dated menus.")
    }

    pub fn read_dated_menus(&self, date: NaiveDate) -> Result<Vec<Menu>> {
        serde_json::from_str(self.read_json_string(format!("/menu/date?date={}", date.and_hms(0, 0, 0).timestamp() * 1000).as_str())?.as_str()).with_context(|| "Failed to parse json from dated menus.")
    }

    pub fn read_api_info(&self) -> Result<ApiInformation> {
        serde_json::from_str(self.read_json_string("")?.as_str()).with_context(|| "Failed to parse json from api information.")
    }

    pub fn read_menus_search(&self, query: &str) -> Result<Vec<Menu>> {
        serde_json::from_str(self.read_json_string(format!("/menu/search?query={}", query).as_str())?.as_str()).with_context(|| "Failed to parse json from api information.")
    }

    pub fn read_menu_amount(&self) -> Result<u32> {
        let value: Value = serde_json::from_str(self.read_json_string("/stats/menu")?.as_str()).with_context(|| "Failed to parse json from menu stats")?;
        Ok(value["amount"].as_u64().with_context(|| "Couldn't find property 'amount' on menu stats endpoint.")? as u32)
    }
}