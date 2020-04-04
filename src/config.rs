use serde::{ Deserialize };
use indexmap::IndexMap;


#[derive(Deserialize)]
pub struct Config {
    pub packages: IndexMap<String, Package>
}


impl Config {
    pub fn from(path: &str) {
        // Try and open the file, if it fails, explode gracefully
    }
}



#[derive(Deserialize)]
pub struct Package {
    pub name: String
}