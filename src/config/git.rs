use serde::{ Deserialize };



#[derive(Clone, Debug, Deserialize)]
pub struct Git {
    repository: String
}


impl Git {
    pub fn url(&self) -> &String {
        &self.repository
    }
}