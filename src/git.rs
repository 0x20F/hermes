use git2::Repository;
use crate::config::Package;


pub struct Git {}

impl Git {
    pub fn clone((name, package): (String, Package)) {
        let output_dir = match package.to {
            Some(p) => format!("{}/{}", p, name),
            None => format!("repositories/{}", name)
        };

        let url = format!("https://github.com/{}", package.git);

        // Remove the directory if it exists
        match std::fs::remove_dir_all(&output_dir) {
            _ => () // Ignore whatever happens
        }

        match Repository::clone(&url, output_dir) {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to clone repo: {}", e)
        };
    }
}