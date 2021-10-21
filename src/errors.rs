use std::io;
use std::num;
use std::str;
use std::string;

use failure::Error;

#[derive(Debug, Fail)]
pub enum MachError {
    #[fail(display = "fail to interpret a sequence of u8 as a string, {}.", _0)]
    Utf8Error(#[cause] str::Utf8Error),
    #[fail(display = "fail to convert a String from a UTF-8 byte vector, {}.", _0)]
    FromUtf8Error(#[cause] string::FromUtf8Error),
    #[fail(display = "fail to parse UUID, {}.", _0)]
    UuidError(::uuid::Error),
    #[fail(display = "fail to do I/O operations, {}.", _0)]
    IoError(#[cause] io::Error),
    #[cfg(feature = "display")]
    #[fail(display = "fail to parse time, {}.", _0)]
    TimeParseError(#[cause] time::error::ComponentRange),
    #[fail(display = "fail to parse integer, {}.", _0)]
    ParseIntError(#[cause] num::ParseIntError),
    #[fail(display = "fail to parse octal, {}.", _0)]
    ParseOctalError(String),
    #[fail(display = "unknown file format, magic 0x{:08x}.", _0)]
    UnknownMagic(u32),
    #[fail(display = "unknown symbol type, {}.", _0)]
    UnknownSymType(u8),
    #[fail(display = "offset {} out of range: [0..{}).", _0, _1)]
    OutOfRange(usize, usize),
    #[fail(display = "number overflowing.")]
    NumberOverflow,
    #[fail(display = "buffer overflowing, {}.", _0)]
    BufferOverflow(usize),
}

impl From<str::Utf8Error> for MachError {
    fn from(err: str::Utf8Error) -> Self {
        MachError::Utf8Error(err)
    }
}

impl From<string::FromUtf8Error> for MachError {
    fn from(err: string::FromUtf8Error) -> Self {
        MachError::FromUtf8Error(err)
    }
}

impl From<uuid::Error> for MachError {
    fn from(err: uuid::Error) -> Self {
        MachError::UuidError(err)
    }
}

impl From<io::Error> for MachError {
    fn from(err: io::Error) -> Self {
        MachError::IoError(err)
    }
}

#[cfg(feature = "display")]
impl From<time::error::ComponentRange> for MachError {
    fn from(err: time::error::ComponentRange) -> Self {
        MachError::TimeParseError(err)
    }
}

impl From<num::ParseIntError> for MachError {
    fn from(err: num::ParseIntError) -> Self {
        MachError::ParseIntError(err)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
