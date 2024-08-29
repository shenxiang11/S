use std::path::PathBuf;
use clap::Parser;
use crate::util::verify_path;

#[derive(Parser, Debug)]
pub struct ServeArgs {
    #[clap(value_parser = verify_path, default_value = ".")]
    pub(crate) directory: PathBuf,

    #[clap(short, long, default_value_t = 3000)]
    pub(crate) port: u16,
}
