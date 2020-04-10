use std::fs;
use remove_dir_all::*;



// Recursively remove directories
pub fn remove_dir(dir: &str) {
    match remove_dir_all(dir) {
        _ => () // Ignore whatever happens
    }
}


// Recursively create directories
pub fn create_dir(dir: &str) {
    match fs::create_dir_all(dir) {
        _ => () // Ignore whatever happens
    }
}


// Create a new file and write text to it
pub fn create_file(dir: &str, text: &str) {
    match fs::write(dir, text) {
        _ => () // Ignore whatever happens
    }
}