use serde::{Deserialize};

use crate::download::{ git, remote };
use crate::config::Config;
use super::github::Github;
use crate::tree;
use crate::event_output::Type;

use std::sync::Arc;
use crate::error::Error;


const DEFAULT_OUTPUT_DIR: &str = "repositories";
const DEFAULT_FILENAME: &str = "no_name_provided";



#[derive(Clone, Debug, Deserialize)]
pub struct Package {
    github: Option<Github>,
    remote: Option<String>,
    directory: Option<String>,
    filename: Option<String>,


    #[serde(skip_deserializing)]
    name: String,

    #[serde(skip_deserializing)]
    config: Arc<Config>
}


impl Package {
    pub fn give(&mut self, name: String, config: Arc<Config>) {
        self.name = name;
        self.config = config;
    }


    pub fn build(&self, fresh: bool) -> Result<(), Error> {
        let output_dir = &self.directory();

        if fresh {
            tree::remove_dir(output_dir);
        }
        tree::create_dir(output_dir);


        println!("Building package: {}", self.name);
        self.download()?;

        if self.config.scripts.is_some() {
            self.run();
        }

        Ok(())
    }


    pub fn directory(&self) -> String {
        match &self.directory {
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


    fn download(&self) -> Result<(), Error> {
        let output_dir = &self.directory();

        if let Some(repo) = &self.github {
            return git::clone(&repo.url(), output_dir);
        }


        let output_file = &self.filename();


        if let Some(url) = &self.remote {
            let format_message: String = format!("Downloaded {}", self.name);
            Type::Done(format_message.as_str()).show();

            return remote::get(url, output_dir, output_file);
        }

        let format_message: String = format!("Failed to download {}", self.name);
        Type::Error(format_message.as_str()).show();

        Ok(())
    }


    fn run(&self) {
        let scripts = self.config.scripts.as_ref();

        for (name, script) in scripts.unwrap() {
            println!("Found script: {}", name);
            script.exec();
        }
    }
}