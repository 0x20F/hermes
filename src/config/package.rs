use serde::{ Deserialize };

use crate::download::{ git, remote };
use crate::tree;

use super::github::Github;
use crate::config::Config;
use std::sync::Arc;


const DEFAULT_OUTPUT_DIR: &str = "repositories";
const DEFAULT_FILENAME: &str = "no_name_provided";



#[derive(Clone, Debug, Deserialize)]
pub struct Package {
    pub github: Option<Github>,
    pub remote: Option<String>,
    directory: Option<String>,
    filename: Option<String>,


    #[serde(skip_deserializing)]
    pub name: String
}


impl Package {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }


    pub fn build(&self, fresh: bool, config: Arc<Config>) -> Result<(), String> {
        let output_dir = &self.directory();

        if fresh {
            tree::remove_dir(output_dir);
        }
        tree::create_dir(output_dir);


        println!("Building package: {}", self.name);
        self.download()?;
        //self.run();

        Ok(())
    }


    pub fn directory(&self) -> String {
        return match &self.directory {
            Some(dir) => dir.clone(),
            _ => format!("{}/{}", DEFAULT_OUTPUT_DIR, self.name)
        }
    }


    pub fn filename(&self) -> String {
        let file: &str = match &self.filename {
            Some(file) => file,
            _ => DEFAULT_FILENAME
        };

        file.to_string()
    }


    fn download(&self) -> Result<(), String> {
        let output_dir = &self.directory();

        println!("\tDownloading package: {}", self.name);

        if let Some(repo) = &self.github {
            return git::clone(&repo.url(), output_dir);
        }


        let output_file = &self.filename();

        if let Some(url) = &self.remote {
            return remote::get(url, output_dir, output_file);
        }

        Ok(())
    }


    fn run(&self) {
        todo!();
    }
}