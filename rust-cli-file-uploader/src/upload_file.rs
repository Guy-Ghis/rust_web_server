use reqwest::blocking::Client;
use std::{fs::File, io::Read, path::Path};

pub fn upload_file<P: AsRef<Path>>(
    file_path: P,
    url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(file_path.as_ref())?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let client = Client::new();

    let response = client.post(url).body(buffer).send()?;

    if response.status().is_success() {
        println!("File uploaded successfully.");
    } else {
        eprintln!("Failed to upload file: {}", response.status());
    }

    Ok(())
}
