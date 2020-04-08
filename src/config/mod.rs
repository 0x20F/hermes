mod github;
mod package;

use indexmap::IndexMap;
use std::fs::read_to_string;
use serde::{ Deserialize };

pub use package::Package;



#[derive(Debug, Deserialize)]
pub struct Config {
    pub packages: IndexMap<String, Package>
}


impl Config {
    pub fn from(path: &str) -> Option<Config> {
        if let Ok(f) = read_to_string(path) {
            // Handle toml failing because it can't find required values?
            return Some(toml::from_str(&f).unwrap());
        }

        None
    }
}














