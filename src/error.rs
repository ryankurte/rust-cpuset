
use std::num::ParseIntError;
use std::str::ParseBoolError;


#[derive(Debug, Fail)]
pub enum Error {
  #[fail(display = "io error: {}", 0)]
  Io(std::io::Error),

  #[fail(display = "CPUSET creation failed")]
  CreationFailed,

  #[fail(display = "Error parsing integer {:?}", 0)]
  InvalidInt(ParseIntError),

  #[fail(display = "Error parsing boolean {:?}", 0)]
  InvalidBool(ParseBoolError),

  #[fail(display = "Invalid format error '{}' (expected {})", 0, 1)]
  InvalidFormat(String, String),
}

impl From<std::io::Error> for Error {
  fn from(e: std::io::Error) -> Self {
    Self::Io(e)
  }
}

impl From<ParseIntError> for Error {
  fn from(e: ParseIntError) -> Self {
    Self::InvalidInt(e)
  }
}

impl From<ParseBoolError> for Error {
  fn from(e: ParseBoolError) -> Self {
    Self::InvalidBool(e)
  }
}
