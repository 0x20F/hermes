mod github;
mod package;
mod script;

use indexmap::IndexMap;
use std::fs::read_to_string;
use serde::{ Deserialize };

use script::Script;
use crate::error::Error;
pub use package::Package;


#[derive(Default, Debug, Deserialize)]
pub struct Config {
    pub packages: IndexMap<String, Package>,
    pub scripts: Option<IndexMap<String, Script>>
}


impl Config {
    pub fn from(path: &str) -> Result<Config, Error> {
        let file = read_to_string(path);

        if file.is_err() {
            return Err(Error::Config);
        }

        // Handle toml failing because it can't find required values?
        let res = toml::from_str::<Config>(&file.unwrap());

        if res.is_err() {
            return Err(Error::Config);
        }

        Ok(res.unwrap())
    }
}














