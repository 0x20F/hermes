use reqwest::blocking;
use crate::tree::create_file;
use paris::{ info };


pub fn get(url: &str, out: &str, filename: &str) -> Result<(), &'static str> {
    let response = match blocking::get(url) {
        Ok(resp) => resp,
        Err(_) => {
            return Err("Could not retrieve the given file"); // TODO: Better message
        }
    };


    if let Ok(text) = response.text() {
        // Save to file
        let output_dir = format!("{}/{}", out, filename);
        create_file(&output_dir, &text);
    } else {
        // Saving failed
        return Err("Could not save to the given file"); // TODO: Better message
    }

    info!("<magenta>Done</> downloading {}", url);

    Ok(())
}
