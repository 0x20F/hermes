mod git;
mod remote;

use crate::config::{Package, Out};
use crate::tree;



const DEFAULT_OUTPUT_DIR: &str = "repositories";



pub fn package(p: Package, fresh: bool) -> Result<(), String> {
    let name = p.name.unwrap();

    let mut output_dir = default_directory(&name);
    let mut filename = "no_filename_provided".to_string();


    if let Some(out) = p.out {
          if let Some(dir) = out.directory {
              output_dir = dir;
          }

        if let Some(file) = out.filename {
            filename = file
        }
    };


    // Cleanup if told to
    if fresh {
        tree::remove_dir(&output_dir);
        tree::create_dir(&output_dir);
    }


    if let Some(repo) = p.github {
        git::clone(&repo.url(), &output_dir)?;
    }


    if let Some(url) = p.remote {
        remote::get(&url, &output_dir, &filename)?;
    }


    Ok(())
}



fn default_directory(package_name: &str) -> String {
    format!("{}/{}", DEFAULT_OUTPUT_DIR, package_name)
}