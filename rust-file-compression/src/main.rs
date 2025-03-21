mod compression;
use std::error::Error;

use compression::CompressFile;
fn main() -> Result<(), Box<dyn Error>> {
    let file = CompressFile::new("file.txt");

    file.compress()?;

    Ok(())
}
