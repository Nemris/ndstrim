use std::fmt;
use std::fs::File;
use std::io::{self, ErrorKind, Read, Seek, SeekFrom};
use std::mem;
use std::path::Path;
use std::result;

use serde::Deserialize;

use crate::crc;

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Deserialization(bincode::Error),
    BadHeader,
    AlreadyTrimmed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{}", e),
            Error::Deserialization(e) => write!(f, "{}", e),
            Error::BadHeader => write!(f, "invalid header"),
            Error::AlreadyTrimmed => write!(f, "already trimmed"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<bincode::Error> for Error {
    fn from(error: bincode::Error) -> Self {
        Error::Deserialization(error)
    }
}

#[derive(Deserialize, PartialEq)]
struct NtrTwlHeader {
    title: [u8; 12],
    gamecode: [u8; 4],
    makercode: [u8; 2],
    unitcode: u8,
    #[serde(with = "serde_arrays")]
    ignored0: [u8; 109],
    ntr_rom_size: u32,
    header_size: u32,
    #[serde(with = "serde_arrays")]
    ignored1: [u8; 56],
    #[serde(with = "serde_arrays")]
    nintendo_logo: [u8; 156],
    nintendo_logo_crc: u16,
    header_crc: u16,
    ignored2: [u8; 32],

    // TWL header half starts here.
    #[serde(with = "serde_arrays")]
    ignored3: [u8; 144],
    ntr_twl_rom_size: u32,
    #[serde(with = "serde_arrays")]
    ignored4: [u8; 3564],
}

impl NtrTwlHeader {
    fn from_file(f: &mut File) -> Result<Self> {
        let mut buf = vec![0; mem::size_of::<Self>()];
        f.read_exact(&mut buf)?;

        let crc = crc::checksum(&buf[..0x15e]);
        let header: Self = bincode::deserialize(&buf)?;
        if header.header_crc != crc || !header.is_logo_valid() {
            return Err(Error::BadHeader);
        }

        Ok(header)
    }

    fn is_logo_valid(&self) -> bool {
        crc::checksum(&self.nintendo_logo) == 0xcf56
    }

    fn is_ntr_only(&self) -> bool {
        self.unitcode == 0x00
    }
}

pub struct NdsFile {
    handle: File,
    file_size: u64,
    trimmed_size: u64,
}

impl NdsFile {
    pub fn open(path: &Path) -> Result<Self> {
        let mut handle = File::options().read(true).write(true).open(path)?;
        let header = NtrTwlHeader::from_file(&mut handle)?;

        let file_size = handle.metadata()?.len();
        let trimmed_size = Self::compute_trimmed_size(&mut handle, &header)?;
        if file_size <= trimmed_size {
            return Err(Error::AlreadyTrimmed);
        }

        Ok(Self {
            handle,
            file_size,
            trimmed_size,
        })
    }

    fn has_cert(handle: &mut File, offset: u64) -> io::Result<bool> {
        const RSA_MAGIC: [u8; 2] = [0x61, 0x63]; // Equals "ac".

        let mut buf = vec![0; 2];
        handle.seek(SeekFrom::Start(offset))?;
        handle.read_exact(&mut buf)?;

        Ok(buf == RSA_MAGIC)
    }

    fn compute_trimmed_size(handle: &mut File, header: &NtrTwlHeader) -> Result<u64> {
        const RSA_SIZE: u64 = 0x88;

        if !header.is_ntr_only() {
            return Ok(header.ntr_twl_rom_size.into());
        }

        let mut trimsize = header.ntr_rom_size.into();

        let has_cert = Self::has_cert(handle, trimsize).map_err(|e| match e.kind() {
            // Assume the file has already been trimmed if EOF is encountered.
            ErrorKind::UnexpectedEof => Error::AlreadyTrimmed,
            _ => e.into(),
        })?;
        if has_cert {
            trimsize += RSA_SIZE;
        }

        Ok(trimsize)
    }

    pub fn trim(&mut self) -> Result<()> {
        self.handle.set_len(self.trimmed_size)?;
        Ok(())
    }

    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    pub fn trimmed_size(&self) -> u64 {
        self.trimmed_size
    }
}
