use crate::error::Error;

use git2::Repository;
use std::path::Path;




pub fn clone(url: &str, out: &str) -> Result<(), Error> {
    println!("\tCloning repo");

    match Repository::clone(&url, &out) {
        Ok(_) => (),
        Err(_) => {
            if !Path::new(&out).exists() {
                return Err(Error::Clone);
            }
        }
    };

    Ok(())
}
