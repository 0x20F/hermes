use serde::{ Deserialize };

use crate::download::{ git, remote };
use crate::tree;

use super::github::Github;



const DEFAULT_OUTPUT_DIR: &str = "repositories";
const DEFAULT_FILENAME: &str = "no_name_provided";



#[derive(Debug, Deserialize)]
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


    pub fn build(&self, fresh: bool) -> Result<(), String> {
        let output_dir = &self.directory();

        if fresh {
            tree::remove_dir(output_dir);
        }
        tree::create_dir(output_dir);

        self.download()?;

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

        if let Some(repo) = &self.github {
            return git::clone(&repo.url(), output_dir);
        }


        let output_file = &self.filename();

        if let Some(url) = &self.remote {
            return remote::get(url, output_dir, output_file);
        }

        Ok(())
    }
}