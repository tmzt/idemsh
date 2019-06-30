
use std::fmt;
use std::result;
use std::io::Error as IOError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorType {
    IOError(Box<IOError>),
    Message(String),
}

pub struct  Error {
    repr: ErrorType
}

impl fmt::Debug for Error {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.repr {
            ErrorType::IOError(ref e) => e.fmt(fmt),
            ErrorType::Message(ref s) => write!(fmt, "{}", s)
        }
    }
}

impl From<IOError> for Error {
    fn from(e: IOError) -> Self {
        Error { repr: ErrorType::IOError(Box::new(e)) }
    }
}
