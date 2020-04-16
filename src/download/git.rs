use crate::error::Error;

use git2::Repository;
use std::path::Path;
use crate::output::{Out, Type};


pub fn clone(url: &str, out: &str) -> Result<(), Error> {
    match Repository::clone(&url, &out) {
        Ok(_) => (),
        Err(_) => {
            if !Path::new(out).exists() {
                return Err(Error::Clone);
            }
        }
    };

    Out::write(Type::Done, &format!("downloading {}", url));

    Ok(())
}
