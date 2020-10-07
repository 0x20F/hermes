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
    name: String,

    #[serde(skip_deserializing)]
    scripts: Option<Arc<HashMap<String, Script>>>
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

        // TODO: Execute script directly?
        self.exec();

        Ok(())
    }


    pub fn exec(&self) -> Result<(), &'static str> {
        let scripts = match self.scripts.clone() {
            Some(s) => s,
            None => return Ok(())
        };


        for name in self.script_keys() {
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


    pub fn with_scripts(&mut self, scripts: Arc<HashMap<String, Script>>) -> &mut Self {
        self.scripts = Some(scripts);
        self
    }


    fn script_keys(&self) -> Vec<String> {
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