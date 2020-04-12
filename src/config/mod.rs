mod github;
mod package;
mod script;

use std::fs::read_to_string;
use serde::{ Deserialize };

use script::Script;
use crate::error::Error;
pub use package::Package;
use std::collections::HashMap;
use std::thread;


#[derive(Default, Debug, Deserialize)]
pub struct Config {
    pub packages: HashMap<String, Package>,
    pub scripts: Option<HashMap<String, Script>>
}


impl Config {
    pub fn from(path: &str) -> Result<Config, Error> {
        let file = read_to_string(path);

        if file.is_err() {
            return Err(Error::Config);
        }

        // Handle toml failing because it can't find required values?
        let res = toml::from_str::<Config>(&file.unwrap());

        if res.is_err() {
            return Err(Error::Config);
        }

        Ok(res.unwrap())
    }


    pub fn build_packages(&self, fresh: bool) -> Vec<Package> {
        let mut threads = vec![];
        let mut survivors = vec![];

        for (name, mut package) in self.packages.clone() {
            threads.push(thread::spawn(move || {
                package.give(name.to_owned());

                // If it's not an error, give back the
                // package so others can use it
                match package.build(fresh) {
                    Ok(_) => Ok(package),
                    Err(e) => Err(e)
                }
            }));
        }

        // Wait for all threads to finish before exiting
        for thread in threads {
            // If thread didn't die, display error or save package
            if let Ok(res) = thread.join() {
                match res {
                    Ok(p) => survivors.push(p),
                    Err(e) => e.display()
                }
            }
        }

        survivors
    }


    pub fn execute_scripts(&self, packages: &[Package]) {
        if let None = self.scripts {
            return;
        }

        let scripts = self.scripts.as_ref().unwrap();

        for package in packages {
            package.exec(scripts);
        }
    }
}














