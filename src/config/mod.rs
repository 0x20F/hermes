mod github;
mod package;
mod script;

use std::fs::read_to_string;
use serde::{ Deserialize };

use script::Script;
use crate::error::Error;
pub use package::Package;
use std::collections::HashMap;


#[derive(Default, Debug, Deserialize)]
pub struct Config {
    pub packages: HashMap<String, Package>,
    pub scripts: Option<HashMap<String, Script>>
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














