use reqwest::blocking;
use crate::error::Error;


pub fn get(url: &str, out: &str, filename: &str) -> Result<(), Error> {
    println!("\tDownloading file");

    let response = match blocking::get(url) {
        Ok(resp) => resp,
        Err(_) => {
            return Err(Error::Remote);
        }
    };


    if let Ok(text) = response.text() {
        // Save to file
        let output_dir = format!("{}/{}", out, filename);
        std::fs::write(output_dir, text).unwrap();
    } else {
        // Saving failed
        return Err(Error::Save);
    }


    Ok(())
}