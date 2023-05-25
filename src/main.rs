mod config;
mod crc;
mod nds;

use std::process;

use config::Config;
use nds::NdsFile;

fn main() {
    let config = Config::new().unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    for file in config.files.iter() {
        let mut ndsfile = match NdsFile::open(file) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("'{}': {}", file.display(), e);
                continue;
            }
        };

        if let Err(e) = ndsfile.trim() {
            eprintln!("'{}': {}", file.display(), e);
            continue;
        };

        println!(
            "'{}': size reduced from {} to {}",
            file.display(),
            ndsfile.file_size(),
            ndsfile.trimmed_size()
        );
    }
}
