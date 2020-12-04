use std::io::{self, prelude::*, Cursor};
use std::fmt;
use codec::bgm::{self, Bgm};
use crate::fs::FileTypes;
use crate::midi;

pub const FILE_TYPES: FileTypes = FileTypes {
    extensions: ".bin .bgm .mid .midi",
    mime_types: "application/octect-stream application/x-bgm audio/midi audio/midi",
};

#[derive(Debug)]
pub enum Error {
    UnsupportedFileType,
    Bgm(bgm::de::Error),
    Smf(midi::smf_to_bgm::Error),
    Io(io::Error),
}

impl From<bgm::de::Error> for Error {
    fn from(source: bgm::de::Error) -> Self {
        Self::Bgm(source)
    }
}

impl From<midi::smf_to_bgm::Error> for Error {
    fn from(source: midi::smf_to_bgm::Error) -> Self {
        Self::Smf(source)
    }
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        Self::Io(source)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnsupportedFileType => write!(f, "unsupported file type (must be BGM or MIDI)"),
            Error::Bgm(source)  => write!(f, "malformed BGM: {}", source),
            Error::Smf(source)  => write!(f, "cannot convert MIDI: {}", source),
            Error::Io(source)   => write!(f, "cannot read file: {}", source),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Bgm(source) => Some(source),
            Error::Smf(source) => Some(source),
            Error::Io(source) => Some(source),
            _ => None,
        }
    }
}

pub fn read_agnostic(raw: &[u8]) -> Result<Bgm, Error> {
    let mut cursor = Cursor::new(raw);

    let mut buffer = [0; 4];
    cursor.read_exact(&mut buffer)?;

    match &buffer {
        // TODO: nice errors
        b"BGM " => Ok(Bgm::from_bytes(raw)?),
        b"MThd" => Ok(midi::smf_to_bgm(raw)?),
        _       => Err(Error::UnsupportedFileType),
    }
}