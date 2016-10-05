mod error;
pub use self::error::*;

mod seq_visitor;
pub use self::seq_visitor::*;

use std::io::Read;
use byteorder;
use byteorder::ByteOrder;
use serde;

pub struct CdrDeserializer<'a, R:'a> {
    data: &'a mut R
}

impl<'a, R:'a> CdrDeserializer<'a,R> {
    pub fn new(incoming: &'a mut R) -> Self {
        CdrDeserializer{ data: incoming }
    }
}

impl<'a,R: Read> serde::Deserializer for CdrDeserializer<'a,R> {
    type Error = CdrDeserializerError;

    fn deserialize<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_bool<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_usize<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_u8<V>(&mut self, mut visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        let mut buf : [u8; 1] = [0; 1];
        try!(self.data.read(&mut buf));
        visitor.visit_u8(buf[0])
    }

    fn deserialize_u16<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_u32<V>(&mut self, mut visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        let mut buf: [u8; 4] = [0; 4];
        try!(self.data.read(&mut buf));
        let val = byteorder::BigEndian::read_u32(&buf[..]);
        visitor.visit_u32(val)
    }

    fn deserialize_u64<V>(&mut self, mut visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        let mut buf: [u8; 8] = [0; 8];
        try!(self.data.read(&mut buf));
        let val = byteorder::BigEndian::read_u64(&buf[..]);
        visitor.visit_u64(val)
    }

    fn deserialize_isize<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_i8<V>(&mut self, mut visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        let mut buf : [u8; 1] = [0; 1];
        try!(self.data.read(&mut buf));
        visitor.visit_i8(buf[0] as i8)
    }

    fn deserialize_i16<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_i32<V>(&mut self, mut visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        let mut buf: [u8; 4] = [0; 4];
        try!(self.data.read(&mut buf));
        let val = byteorder::BigEndian::read_i32(&buf[..]);
        visitor.visit_i32(val)
    }

    fn deserialize_i64<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_f32<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_f64<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_char<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_str<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_string<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_unit<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_option<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_seq<V>(&mut self, mut visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        let mut buf: [u8; 4] = [0; 4];
        try!(self.data.read(&mut buf));
        let len = byteorder::BigEndian::read_u32(&buf[..]) as usize;

        let seq_visitor = CdrSeqVisitor::new(self,len, true);
        visitor.visit_seq(seq_visitor)
    }

    fn deserialize_seq_fixed_size<V>(&mut self, len: usize, mut visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        let seq_visitor = CdrSeqVisitor::new(self,len, false);
        visitor.visit_seq(seq_visitor)
    }

    fn deserialize_bytes<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_map<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(&mut self, _: &'static str, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(&mut self, _: &'static str, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(&mut self, _: &'static str, _: usize, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_struct<V>(&mut self, _: &'static str, fields: &'static [&'static str], mut visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        let seq_visitor = CdrSeqVisitor::new(self,fields.len(),false);
        visitor.visit_seq(seq_visitor)
    }

    fn deserialize_struct_field<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_tuple<V>(&mut self, _: usize, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_enum<V>(&mut self, _: &'static str, _: &'static [&'static str], _: V) -> Result<V::Value, Self::Error> where V: serde::de::EnumVisitor {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }
}