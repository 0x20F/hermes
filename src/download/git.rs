use git2::Repository;
use std::path::Path;
use paris::{ log };


pub fn clone(url: &str, out: &str) -> Result<(), &'static str> {
    match Repository::clone(&url, &out) {
        Ok(_) => (),
        Err(_) => {
            if !Path::new(out).exists() {
                return Err("Could not clone repository"); // TODO: Better message
            }
        }
    };

    log!("<magenta>Done</> downloading {}", url);

    Ok(())
}
