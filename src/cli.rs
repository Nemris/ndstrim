//! Structs to handle command-line argument parsing.

#![warn(clippy::pedantic)]

use std::path::PathBuf;

use clap::Parser;

/// Command-line arguments.
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// ROM files to trim
    #[arg(required = true)]
    pub files: Vec<PathBuf>,

    /// Simulate execution, don't trim
    #[arg(short, long)]
    pub simulate: bool,

    /// Extension for trimmed files
    #[arg(short, long, default_value_t = String::from("trim.nds"))]
    pub extension: String,

    /// Trim files in-place
    #[arg(short, long)]
    pub inplace: bool,
}
