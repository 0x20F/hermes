use serde::{ Deserialize };
use indexmap::IndexMap;
use std::fs::read_to_string;




const GITHUB_HOST: &str = "https://github.com/";





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




#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: Option<String>,
    pub github: Option<Github>,
    pub remote: Option<String>,
    pub to: Option<String>
}


impl Package {
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
}




#[derive(Debug, Deserialize)]
pub struct Github {
    username: String,
    repository: String
}


impl Github {
    pub fn url(&self) -> String {
        format!("{}/{}/{}", GITHUB_HOST, self.username, self.repository)
    }
}