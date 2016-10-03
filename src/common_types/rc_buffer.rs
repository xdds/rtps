use serde;

use std::sync::Arc;
use std::borrow::Borrow;

#[derive(Debug, Clone, PartialEq)]
pub struct ArcBuffer {
    buf: Arc<Vec<u8>>
}

impl ArcBuffer {
    pub fn from_vec(vec: Vec<u8>) -> Self {
        ArcBuffer { buf: Arc::new(vec) }
    }

    pub fn buf(&self) -> &[u8] {
        let borrow : &Vec<u8> = self.buf.borrow();
        borrow.borrow()
    }

    pub fn len(&self) -> usize {
        (self.buf.borrow() : &Vec<u8>).len()
    }
}

impl serde::Deserialize for ArcBuffer {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
        let data : Vec<u8> = try!(serde::Deserialize::deserialize(deserializer));
        Ok(ArcBuffer::from_vec(data))
    }
}