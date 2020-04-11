use std::fs;
use remove_dir_all::*;



// Recursively remove directories
pub fn remove_dir(dir: &str) {
    let _ = remove_dir_all(dir);

    ()
}


// Recursively create directories
pub fn create_dir(dir: &str) {
    let _ = fs::create_dir_all(dir);

    ()
}


// Create a new file and write text to it
pub fn create_file(dir: &str, text: &str) {
    let _ = fs::write(dir, text);

    ()
}