use std;
use serde;

use super::CdrDeserializerError;

pub struct CdrSeqVisitor<'a, T> where T: Sized + 'a + serde::Deserializer  {
    pub deserializer: &'a mut T
}

impl<'a,T> CdrSeqVisitor<'a,T> where T: serde::Deserializer {
    pub fn new(thing: &'a mut T) -> Self {
        CdrSeqVisitor{ deserializer: thing }
    }
}

impl<'a,R> serde::de::SeqVisitor for CdrSeqVisitor<'a,R> where R: serde::Deserializer, CdrDeserializerError: std::convert::From<<R as serde::Deserializer>::Error> {
    type Error = CdrDeserializerError;

    fn visit<T>(&mut self) -> Result<Option<T>,Self::Error> where T: serde::Deserialize {
        let value = try!(serde::Deserialize::deserialize(self.deserializer));
        Ok(Some(value))
    }

    fn end(&mut self) -> Result<(),Self::Error> {
        Ok(())
    }
}