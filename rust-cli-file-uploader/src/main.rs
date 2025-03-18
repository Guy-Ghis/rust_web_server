mod upload_file;
use upload_file::upload_file;
use clap::Command;
use clap::Arg;
fn main() {
    let matches = Command::new("Rust CLI File Uploader")
    .version("1.0")
    .author("Guy-Ghis <guyghis@gmail.com>")
    .about("Uploads files to a specified endpoint")
    .arg(
        Arg::new("file")
        .help("The path to the file upload")
        .required(true)
        .index(1),
    )
    .arg(
        Arg::new("url")
        .help("The upload endpoint URL")
        .required(true)
        .index(2),
    )
    .get_matches();

    let file_path = matches.get_one::<String>("file").expect("file path not found");
    let url = matches.get_one::<String>("url").expect("URL not found");

    if let Err(e) = upload_file(file_path, url) {
        eprintln!("Error uploading file: {}", e);
    }
}
