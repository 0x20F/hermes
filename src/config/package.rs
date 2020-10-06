use serde::{Deserialize};

use crate::download::{ git, remote };
use super::git::Git;
use crate::tree;
use std::sync::Arc;
use paris::{ log };

use crate::config::script::Script;
use std::collections::HashMap;


const DEFAULT_OUTPUT_DIR: &str = "repositories";
const DEFAULT_FILENAME: &str = "no_name_provided";




#[derive(Clone, Debug, Deserialize)]
pub struct Package {
    // TODO: One or the other, either git or remote
    git: Option<Git>,
    remote: Option<String>,

    directory: Option<String>, // TODO: Directory only for entire repos
    filename: Option<String>, // TODO: Filename only for raw file downloads

    exec: Option<Vec<String>>,


    #[serde(skip_deserializing)]
    name: String
}



impl Package {
    pub fn get_name(&self) -> &String {
        &self.name
    }


    pub fn build(&self, fresh: bool) -> Result<(), &'static str> {
        let output_dir = &self.directory();

        Package::clean(output_dir, fresh);


        log!("<bright green>Building</> {}", self.get_name());


        if let Some(repo) = &self.git {
            git::clone(&repo.url(), output_dir)?;
        }

        if let Some(url) = &self.remote {
            remote::get(url, output_dir, &self.filename())?;
        }

        Ok(())
    }


    pub fn exec(&self, scripts: &HashMap<String, Script>) -> Result<(), &'static str> {
        for name in self.scripts() {
            let script = scripts.get(&name).unwrap();

            script.exec(self);
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


    pub fn full_path(&self) -> String {
        format!("{}/{}", tree::current_dir(), self.directory())
    }


    fn scripts(&self) -> Vec<String> {
        match self.exec.as_ref() {
            Some(v) => v.clone(),
            None => vec![]
        }
    }


    fn clean(what: &str, fresh: bool) {
        if fresh {
            tree::remove_dir(what);
        }

        tree::create_dir(what);
    }
}