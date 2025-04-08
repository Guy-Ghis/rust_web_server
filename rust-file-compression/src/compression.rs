extern crate flate2;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::fs::{File, create_dir_all};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub fn compress_file(input: &str, level: u32) -> Result<String, io::Error> {
    let input_path = Path::new(input);

    // Read the entire input file into a vector of bytes
    let mut input_data = Vec::new();
    File::open(input_path)?.read_to_end(&mut input_data)?;

    // Compress the input data
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::new(level));
    encoder.write_all(&input_data)?;
    let compressed_data = encoder.finish()?; // Get the compressed data

    // Create the "../compressed_files" directory if it doesn't exist
    let compressed_dir = Path::new("../compressed_files");
    if !compressed_dir.exists() {
        create_dir_all(compressed_dir)?; // Create the directory and any intermediate directories if needed
    }

    // Change the extension of the file to .gz (replace original extension)
    let output_file_name = input_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        + ".gz";

    let output_path: PathBuf = compressed_dir.join(&output_file_name);

    // Write the compressed data to the output file
    let mut output = File::create(&output_path)?;
    output.write_all(&compressed_data)?;

    // Log the file compression success with full output path
    println!(
        "File compressed successfully. Output saved as: {:?}",
        output_path
    );

    // Return only the file name with the new .gz extension
    Ok(output_file_name)
}
