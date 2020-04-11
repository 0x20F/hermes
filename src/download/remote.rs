use reqwest::blocking;
use crate::error::Error;
use crate::tree::create_file;
use crate::event_output::Type;


pub fn get(url: &str, out: &str, filename: &str) -> Result<(), Error> {
    let response = match blocking::get(url) {
        Ok(resp) => resp,
        Err(_) => {
            return Err(Error::Remote);
        }
    };


    if let Ok(text) = response.text() {
        // Save to file
        let output_dir = format!("{}/{}", out, filename);
        create_file(&output_dir, &text);
    } else {
        // Saving failed
        return Err(Error::Save);
    }

    let format_message: String = format!("Downloaded {}", filename);
    Type::Done(format_message.as_str()).show();

    Ok(())
}