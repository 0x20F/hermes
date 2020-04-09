mod github;
mod package;
mod script;

use indexmap::IndexMap;
use std::fs::read_to_string;
use serde::{ Deserialize };

use script::Script;
pub use package::Package;



#[derive(Debug, Deserialize)]
pub struct Config {
    pub packages: IndexMap<String, Package>,
    pub scripts: Option<IndexMap<String, Script>>
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














