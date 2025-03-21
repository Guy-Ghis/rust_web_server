use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

pub struct CompressFile<'a> {
    file: &'a str,
}

impl<'a> CompressFile<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Self { file: file_path }
    }

    pub fn compress(&self) -> Result<File, Box<dyn Error>> {
        let mut input_file = File::open(&self.file)?;
        let mut buffer = Vec::new();

        input_file.read_to_end(&mut buffer)?;

        let mut compress = ZlibEncoder::new(Vec::new(), Compression::default());

        compress.write_all(&buffer)?;

        let compressed_bytes = compress.finish()?;

        let mut compressed_file = File::create(format!("{:?}.zlib", &self.file))?;

        compressed_file.write_all(&compressed_bytes)?;

        Ok(compressed_file)
    }
}
