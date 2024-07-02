use core::fmt::{self, Debug, Display};
use std::error;

pub(crate) enum ErrorKind {
    InvalidIncrementLevel(String),
}

pub struct Error {
    pub(crate) kind: ErrorKind,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::InvalidIncrementLevel(level) => {
                write!(formatter, "invalid attempted increment level {level}")
            }
        }
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Error(\"")?;
        Display::fmt(self, formatter)?;
        formatter.write_str("\")")?;
        Ok(())
    }
}
