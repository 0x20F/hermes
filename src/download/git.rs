use git2::Repository;
use std::path::Path;


pub struct Git {}

impl Git {
    pub fn clone(url: String, out: String, fresh: bool) -> Result<(), String> {
        // TODO: Update this when using something outside of github
        let url = format!("https://github.com/{}", url);

        // Remove the directory if it exists
        if fresh {
            match std::fs::remove_dir_all(&out) {
                _ => () // Ignore whatever happens
            }
        }

        match Repository::clone(&url, &out) {
            Ok(_) => (),
            Err(e) => {
                if !Path::new(&out).exists() {
                    return Err(format!("Failed to clone repo: {}", url));
                }
            }
        };

        Ok(())
    }
}