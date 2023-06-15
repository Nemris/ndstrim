//! Structs and methods for working with Nintendo DS(i) ROMs.

#![warn(clippy::pedantic)]

use std::fmt;
use std::fs::File;
use std::io::{self, ErrorKind, Read, Seek, SeekFrom};
use std::mem;
use std::path::Path;
use std::result;

use serde::Deserialize;

use crate::crc;

type Result<T> = result::Result<T, Error>;

/// A list of errors that may originate in this module.
#[derive(Debug)]
pub enum Error {
    /// An error occurred during I/O operations.
    Io(io::Error),
    /// Deserializing binary data failed.
    Deserialization(bincode::Error),
    /// The header in the NDS file is malformed.
    BadHeader,
    /// The NDS file is already trimmed.
    AlreadyTrimmed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{e}"),
            Error::Deserialization(e) => write!(f, "{e}"),
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

/// The header of an NDS file.
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

/// An NDS ROM header.
impl NtrTwlHeader {
    /// Loads a header from an open NDS ROM and verifies it.
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

    /// Verifies `self`.
    fn is_logo_valid(&self) -> bool {
        crc::checksum(&self.nintendo_logo) == 0xcf56
    }

    /// Checks whether `self` belongs to an NTR-only ROM.
    fn is_ntr_only(&self) -> bool {
        self.unitcode == 0x00
    }
}

/// An NDS file.
#[allow(clippy::module_name_repetitions)]
pub struct NdsFile {
    /// A handle to the open file.
    handle: File,
    /// The file's on-disk size.
    file_size: u64,
    /// The size of the ROM data.
    trimmed_size: u64,
}

impl NdsFile {
    /// Opens an NDS file for reading and writing.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use nds::NdsFile;
    ///
    /// let path = PathBuf::from("foo.nds");
    /// let ndsfile = NdsFile::open(&path)?;
    /// ```
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

    /// Checks whether the ROM contains RSA magic bytes.
    ///
    /// This is only relevant in certain ROMs, e.g. Mario Kart, for Download Play functionality.
    fn has_cert(handle: &mut File, offset: u64) -> io::Result<bool> {
        const RSA_MAGIC: [u8; 2] = [0x61, 0x63]; // Equals "ac".

        let mut buf = vec![0; 2];
        handle.seek(SeekFrom::Start(offset))?;
        handle.read_exact(&mut buf)?;

        Ok(buf == RSA_MAGIC)
    }

    /// Computes the size of the ROM contents.
    ///
    /// Generally, this matches the size reported in the header, unless the ROM contains a RSA
    /// certificate.
    /// In such a case, the size should include 0x88 more bytes to preserve Download Play.
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

    /// Trims `self` in-place. This is irreversible.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use nds::NdsFile;
    ///
    /// let path = PathBuf::from("foo.nds");
    /// let ndsfile = NdsFile::open(&path)?;
    ///
    /// ndsfile.trim()?;
    /// ```
    pub fn trim(&mut self) -> Result<()> {
        self.handle.set_len(self.trimmed_size)?;
        Ok(())
    }

    /// Copies `self`'s data into `dest`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use nds::NdsFile;
    ///
    /// let src = PathBuf::from("foo.nds");
    /// let dest = PathBuf::from("bar.nds");
    /// let ndsfile = NdsFile::open(&src)?;
    ///
    /// ndsfile.trim_with_name(&dest)?;
    /// ```
    pub fn trim_with_name(&mut self, dest: &Path) -> Result<()> {
        let mut out = File::create(dest)?;
        self.handle.seek(SeekFrom::Start(0))?;
        io::copy(&mut self.handle.by_ref().take(self.trimmed_size), &mut out)?;
        Ok(())
    }

    /// Returns `self`'s on-disk file size.
    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    /// Returns `self`'s content size.
    pub fn trimmed_size(&self) -> u64 {
        self.trimmed_size
    }
}
