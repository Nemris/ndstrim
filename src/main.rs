//! Utility to trim Nintendo DS(i) ROMs.

#![warn(clippy::pedantic)]

mod cli;
mod crc;
mod nds;

use clap::Parser;

use cli::Cli;
use nds::NdsFile;

fn main() {
    let cli = Cli::parse();

    for src in cli.files.iter() {
        let dest = if cli.inplace {
            src.clone()
        } else {
            src.with_extension(&cli.extension)
        };

        let mut ndsfile = match NdsFile::open(src) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("'{}': {}", src.display(), e);
                continue;
            }
        };

        if !cli.simulate {
            if cli.inplace {
                if let Err(e) = ndsfile.trim() {
                    eprintln!("'{}': {}", src.display(), e);
                    continue;
                }
            } else if let Err(e) = ndsfile.trim_with_name(&dest) {
                eprintln!("'{}': {}", src.display(), e);
                continue;
            }
        }

        println!(
            "'{}': size reduced from {} to {}",
            dest.display(),
            ndsfile.file_size(),
            ndsfile.trimmed_size()
        );
    }
}
