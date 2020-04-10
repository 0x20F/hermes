use serde::Deserialize;


#[derive(Clone, Debug, Deserialize)]
pub struct Script {
    script: String
}


impl Script {
    pub fn exec(&self) {
        println!("Running script: {}", self.script);
    }
}