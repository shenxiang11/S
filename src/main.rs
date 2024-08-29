mod util;
mod directory_interceptor;
mod cli;
mod fs_server;

use clap::{Parser};
use crate::cli::ServeArgs;
use crate::fs_server::start_file_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ServeArgs::parse();

    start_file_server(args.directory, args.port).await?;

    Ok(())
}
