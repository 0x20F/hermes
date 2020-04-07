use git2::Repository;
use std::path::Path;


pub fn clone(url: &str, out: &str) -> Result<(), String> {
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
