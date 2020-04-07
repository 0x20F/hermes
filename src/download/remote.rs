use reqwest::blocking;


pub fn get(url: &str, out: &str, filename: &str) -> Result<(), String> {
    let response = match blocking::get(url) {
        Ok(resp) => resp,
        Err(_) => {
            let message = format!("Failed download for: {}", url);
            return Err(message);
        }
    };


    if let Ok(text) = response.text() {
        // Save to file
        let output_dir = format!("{}/{}", out, filename);
        std::fs::write(output_dir, text).unwrap();
    } else {
        let message = format!("Failed saving download at: {}", out);
        return Err(message);
    }


    Ok(())
}