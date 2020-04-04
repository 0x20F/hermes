use serde::{ Deserialize };
use indexmap::IndexMap;


#[derive(Deserialize)]
pub struct Config {
    pub packages: IndexMap<String, Package>
}


#[derive(Deserialize)]
pub struct Package {
    pub name: String
}