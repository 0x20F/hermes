use serde::Deserialize;


#[derive(Clone, Debug, Deserialize)]
pub struct Script {
    script: String
}