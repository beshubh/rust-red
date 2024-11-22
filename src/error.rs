use std;
use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),

    Eof,
    Syntax,
    ExpectedCRLF,
    ExpectedArray,
    ExpectedInteger,
    ExpectedSimpleString,
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Message(msg) => f.write_str(&msg),
            Error::Eof => f.write_str("unexpected end of input"),
            Error::Syntax => f.write_str("syntax does not follow RESP"),
            Error::ExpectedCRLF => f.write_str("expected (CRLF)/\r/\n in the end"),
            Error::ExpectedArray => f.write_str("invalid content expected an array"),
            Error::ExpectedInteger => f.write_str("invalid content expected an integer"),
            Error::ExpectedSimpleString => f.write_str("invalid content expected simple strings"),
        }
    }
}

impl std::error::Error for Error {}
