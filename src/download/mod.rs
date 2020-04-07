mod git;
mod remote;

use crate::config::Package;
use crate::tree;


const DEFAULT_OUTPUT_DIR: &str = "repositories";



pub fn package(p: Package, fresh: bool) -> Result<(), String> {
    let name = p.name.unwrap();

    // Check if package has git or remote
    // Download accordingly
    let output_dir = match p.out {
        Some(path) => path,
        None => format!("{}/{}", DEFAULT_OUTPUT_DIR, name)
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
        remote::get(&url, &output_dir)?;
    }


    Ok(())
}
