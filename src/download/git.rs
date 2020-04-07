use git2::Repository;
use std::path::Path;


pub fn clone(url: &str, out: &str, fresh: bool) -> Result<(), String> {
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
