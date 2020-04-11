use serde::Deserialize;
use paris::log;
use shells::{ sh };



#[derive(Clone, Debug, Deserialize)]
pub struct Script {
    script: String,

    #[serde(skip_deserializing)]
    name: String
}


impl Script {
    pub fn give(&mut self, name: &str) {
        self.name = name.to_owned();
    }


    pub fn exec(&self) {
        log!("<magenta>Running</>: {}", self.name);

        let (_, stdout, _) = sh!("{}", self.script);
        self.display(&stdout);
    }


    pub fn display(&self, output: &str) {
        println!("{}", output);
    }
}