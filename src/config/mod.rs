mod git;
mod package;
mod script;

use std::fs::read_to_string;
use serde::{ Deserialize };

use script::Script;
pub use package::Package;
use std::collections::HashMap;
use std::thread;
use std::sync::Arc;
use paris::{ error };




#[derive(Default, Debug, Deserialize)]
pub struct Config {
    pub packages: HashMap<String, Arc<Package>>,
    pub scripts: Option<HashMap<String, Script>>
}


impl Config {
    pub fn from(path: &str) -> Result<Config, &'static str> {
        let file = read_to_string(path);

        if file.is_err() {
            return Err("Could not find the given configuration file"); // TODO: Better message
        }

        match toml::from_str::<Config>(&file.unwrap()) {
            Ok(config) => Ok(config),
            Err(_) => return Err("Could not parse the config file") // TODO: Better message
        }
    }


    pub fn build_packages(&self, fresh: bool) -> Vec<Arc<Package>> {
        let (mut threads, mut survivors) = (vec![], vec![]);

        // For every package
        for (_, package) in &self.packages {
            let package = package.clone(); // Clone for threading
            let fresh = fresh.clone();

            threads.push(thread::spawn(move || {
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
            // If thread didn't die, save package otherwise display error
            if let Ok(res) = thread.join() {
                match res {
                    Ok(p) => survivors.push(p),
                    Err(_) => error!("Could not build package")
                }
            }
        }

        survivors
    }


    pub fn execute_scripts(&self, packages: Vec<Arc<Package>>) {
        if self.scripts.is_none() {
            return;
        }

        let scripts = self.scripts.as_ref().unwrap();

        for package in packages {
            println!("{:p}", package);
            package.exec(scripts);
        }
    }
}














