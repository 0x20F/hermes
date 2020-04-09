use serde::Deserialize;
use std::str::FromStr;


#[derive(Clone, Debug, Deserialize)]
pub struct Script {
    script: String
}


impl FromStr for Script {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { script: s.to_owned() })
    }
}