mod git;
mod remote;

use crate::config::Package;




pub fn download_package(p: &Package) -> Result<(), String> {
    let output_directory = p.directory();

    if let Some(repo) = p.github.as_ref() {
        git::clone(&repo.url(), &output_directory)?;
    }


    let output_filename = p.filename();

    if let Some(url) = p.remote.as_ref() {
        remote::get(&url, &output_directory, &output_filename)?;
    }


    Ok(())
}