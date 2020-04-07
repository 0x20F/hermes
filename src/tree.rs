use std::fs;



// Recursively remove directories
pub fn remove_dir(dir: &str) {
    match fs::remove_dir_all(dir) {
        _ => () // Ignore whatever happens
    }
}


// Recursively create directories
pub fn create_dir(dir: &str) {
    match fs::create_dir_all(dir) {
        _ => () // Ignore whatever happens
    }
}