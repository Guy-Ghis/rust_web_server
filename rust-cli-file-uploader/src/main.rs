use std::error::Error;

use cli_upload::CliUploader;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = CliUploader::from_args();

    cli.upload_file().await?;

    Ok(())
}

mod cli_upload;
