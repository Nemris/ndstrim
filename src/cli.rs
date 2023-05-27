use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// ROM files to trim
    #[arg(required = true)]
    pub files: Vec<PathBuf>,
}
