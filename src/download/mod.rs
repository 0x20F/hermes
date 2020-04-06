mod git;
mod remote;

use crate::config::Package;



pub fn package(p: Package, fresh: bool) -> Result<(), String> {
    let name = p.name.unwrap();

    // Check if package has git or remote
    // Download accordingly
    let output_dir = match p.to {
        Some(path) => format!("{}/{}", path, name),
        None => format!("repositories/{}", name)
    };

    if let Some(url) = p.git {
        git::clone(url, output_dir, fresh)?;
    }

    Ok(())
}
