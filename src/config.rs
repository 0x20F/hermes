use serde::{ Deserialize };
use indexmap::IndexMap;


#[derive(Deserialize)]
pub struct Config {
    pub packages: IndexMap<String, Package>
}


#[derive(Deserialize)]
struct Package {
    name: String
}