use git2::Repository;
use crate::config::Package;
use std::path::Path;


pub struct Git {}

impl Git {
    pub fn clone((name, package): (String, Package), replace: bool) {
        let output_dir = match package.to {
            Some(p) => format!("{}/{}", p, name),
            None => format!("repositories/{}", name)
        };

        let url = format!("https://github.com/{}", package.git);

        // Remove the directory if it exists
        if replace {
            match std::fs::remove_dir_all(&output_dir) {
                _ => () // Ignore whatever happens
            }
        }

        match Repository::clone(&url, &output_dir) {
            Ok(_) => (),
            Err(e) => {
                if !Path::new(&output_dir).exists() {
                    panic!("Failed to clone repo: {}", e);
                }
            }
        };
    }
}