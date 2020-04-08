use serde::{ Deserialize };



const GITHUB_HOST: &str = "https://github.com/";


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