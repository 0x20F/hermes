use reqwest::blocking;
use std::fs::File;
use std::io::Write;


pub fn get(url: &str, out: &str, fresh: bool) -> Result<(), String> {
    let response = match blocking::get(url) {
        Ok(resp) => resp,
        Err(_) => {
            let message = format!("Failed download for: {}", url);
            return Err(message);
        }
    };


    let pieces: Vec<&str> = url.split("/").collect();
    let file_name = pieces.last().unwrap();
    let file_path = format!("{}/{}", out, file_name);


    if let Ok(text) = response.text() {
        // Save to file
        std::fs::create_dir_all(out);
        std::fs::write(file_path, text).unwrap();
    } else {
        let message = format!("Failed saving download at: {}", out);
        return Err(message);
    }


    Ok(())
}