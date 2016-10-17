use std;
use std::fmt::Display;

use serde;

#[derive(Debug)]
pub struct CdrDeserializerError {
    thing: String
}

impl Display for CdrDeserializerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.thing)
    }
}

impl serde::Error for CdrDeserializerError {
    fn custom<T: Into<String>>(msg: T) -> Self {
        CdrDeserializerError{ thing: msg.into() }
    }

    fn end_of_stream() -> Self {
        unimplemented!()
    }
}

impl std::error::Error for CdrDeserializerError {
    fn description(&self) -> &str {
        ""
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

impl From<serde::de::value::Error> for CdrDeserializerError {
    fn from(err: serde::de::value::Error) -> CdrDeserializerError {
        CdrDeserializerError{
            thing: format!("{:?}", err)
        }
    }
}

impl From<std::io::Error> for CdrDeserializerError {
    fn from(err: std::io::Error) -> CdrDeserializerError {
        CdrDeserializerError{
            thing: format!("{:?}", err)
        }
    }
}

impl Into<std::io::Error> for CdrDeserializerError {
    fn into(self) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, self.thing)
    }
}