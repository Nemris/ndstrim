use std::env;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct UsageMessage {
    program: String,
}

impl Error for UsageMessage {}

impl fmt::Display for UsageMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Usage: {} file ...", self.program)
    }
}

pub struct Config {
    pub files: Vec<PathBuf>,
}

impl Config {
    pub fn new() -> Result<Self, UsageMessage> {
        let mut args = env::args();
        let program = args.next().expect("the program name should be available");
        if args.len() < 1 {
            return Err(UsageMessage { program });
        }

        Ok(Self {
            files: args.map(PathBuf::from).collect(),
        })
    }
}
