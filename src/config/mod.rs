mod git;
mod package;
mod script;

use serde::{ Deserialize };

use script::Script;
pub use package::Package;
use std::collections::HashMap;
use std::thread;
use std::sync::Arc;
use paris::{ error };
use std::thread::JoinHandle;


#[derive(Default, Debug, Deserialize)]
pub struct Config {
    pub packages: HashMap<String, Arc<Package>>,
    pub scripts: Option<HashMap<String, Script>>,

    #[serde(skip_deserializing)]
    pub refresh: bool
}


impl Config {
    pub fn build_packages(&mut self) -> Result<&Self, &'static str> {
        let mut threads = vec![];

        // For every package
        for (name, package) in &self.packages {
            let package = package.clone(); // Clone for threading
            let fresh = self.refresh.clone();
            let name = name.clone();

            threads.push(thread::spawn(move || {
                // If it's not an error, give back the
                // package so others can use it
                if let Err(_) = package.build(fresh) {
                    return Err(name);
                }

                Ok(())
            }));
        }

        Self::wait_for_threads(threads, &mut self.packages);

        Ok(self)
    }


    pub fn wait_for_threads(
        threads: Vec<JoinHandle<Result<(), String>>>,
        packages: &mut HashMap<String, Arc<Package>>
    ) {
        // Wait for all threads to finish before exiting
        for thread in threads {
            let res = thread.join();

            if let Err(name) = res.unwrap() {
                error!("Could not build package");
                packages.remove_entry(&name);
            }
        }
    }


    pub fn execute_scripts(&self) {
        if self.scripts.is_none() {
            return;
        }

        let scripts = self.scripts.as_ref().unwrap();

        for (_, package) in &self.packages {
            println!("{:p}", package);
            package.exec(scripts);
        }
    }


    pub fn load_fresh(&mut self, should: bool) -> &mut Self {
        self.refresh = should;
        self
    }
}














