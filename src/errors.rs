use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::str::Utf8Error;


#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    Utf8(Utf8Error),
    Int(ParseIntError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Io(ref e) => e.fmt(f),
            ParseError::Utf8(ref e) => e.fmt(f),
            ParseError::Int(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ParseError::Io(ref e) => Some(e),
            ParseError::Utf8(ref e) => Some(e),
            ParseError::Int(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::Io(err)
    }
}

impl From<Utf8Error> for ParseError {
    fn from(err: Utf8Error) -> ParseError {
        ParseError::Utf8(err)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::Int(err)
    }
}
