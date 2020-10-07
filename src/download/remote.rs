use reqwest::blocking;
use crate::tree::create_file;
use paris::{ log };


pub fn get(url: &str, out: &str, filename: &str) -> Result<(), &'static str> {
    let response = blocking::get(url);

    if let Err(_) = response {
        return Err("Could not retrieve the given file"); // TODO: Better message
    }


    let text = response.unwrap().text();

    if let Err(_) = text {
        return Err("Could not save to the given file"); // TODO: Better message
    }


    // Save to file
    let output_dir = format!("{}/{}", out, filename);
    create_file(&output_dir, &text.unwrap()); // TODO: Throw an error if saving didn't work


    log!("<magenta>Done</> downloading {}", url);
    Ok(())
}
