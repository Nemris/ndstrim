mod cli;
mod crc;
mod nds;

use clap::Parser;

use cli::Cli;
use nds::NdsFile;

fn main() {
    let cli = Cli::parse();

    for file in cli.files.iter() {
        let mut ndsfile = match NdsFile::open(file) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("'{}': {}", file.display(), e);
                continue;
            }
        };

        if !cli.simulate {
            if let Err(e) = ndsfile.trim() {
                eprintln!("'{}': {}", file.display(), e);
                continue;
            };
        }

        println!(
            "'{}': size reduced from {} to {}",
            file.display(),
            ndsfile.file_size(),
            ndsfile.trimmed_size()
        );
    }
}
