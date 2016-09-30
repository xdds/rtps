use serde;
use serde::{ Deserialize, Error };
use std::io::{ Read, Cursor };
use std;
use std::fmt::Display;
use std::convert::From;

use rtps;

pub struct CdrDeserializer<'a, R:'a> {
    data: &'a mut R
}

impl<'a, R:'a> CdrDeserializer<'a,R> {
    pub fn new(incoming: &'a mut R) -> Self {
        CdrDeserializer{ data: incoming }
    }
}

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

struct CdrSeqVisitor<'a, T> where T: Sized + 'a + serde::Deserializer  {
    deserializer: &'a mut T
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

impl<'a,R: Read> serde::Deserializer for CdrDeserializer<'a,R> {
    type Error = CdrDeserializerError;

    fn deserialize<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_bool<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_usize<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_u8<V>(&mut self, mut visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        let mut buf : [u8; 1] = [0; 1];
        try!(self.data.read(&mut buf));
        visitor.visit_u8(buf[0])
    }

    fn deserialize_u16<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_u32<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_u64<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_isize<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_i8<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_i16<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_i32<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_i64<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_f32<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_f64<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_char<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_str<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_string<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_unit<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_option<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_seq<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_seq_fixed_size<V>(&mut self, len: usize, mut visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        let seq_visitor = CdrSeqVisitor::new(self);
        visitor.visit_seq(seq_visitor)
    }

    fn deserialize_bytes<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_map<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(&mut self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(&mut self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(&mut self, name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_struct<V>(&mut self, name: &'static str, fields: &'static [&'static str], mut visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        let seq_visitor = CdrSeqVisitor::new(self);
        visitor.visit_seq(seq_visitor)
    }

    fn deserialize_struct_field<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_tuple<V>(&mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_enum<V>(&mut self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::EnumVisitor {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(&mut self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }
}

#[derive(Deserialize,Debug,PartialEq)]
pub struct Message {
    junk: [u8; 4]
//    submessages: Vec<Submessage>
}

#[derive(Debug,PartialEq)]
pub struct Submessage {
//    id: rtps::SubmessageId,
//    endianness: rtps::CdrEndianness,
//    buf: rtps::common_types::ArcBuffer
}

#[test]
fn bang() {
    let bytes = [
        82, 84, 80, 83, // "RTPS"
        20, 10, // Protocol Type
        86, 19, // Vendor id
        0, 0, 0, 1, // Submessage count
        21, 1, // Submessage 0 headers
        0, 0, 0, 4, // Submessage 0 len
        1, 2, 3, 4  // Submessage 0 data
    ];
    let mut cursor = Cursor::new(bytes);

    let mut de = CdrDeserializer::new(&mut cursor);
    let message : Message = Deserialize::deserialize(&mut de).unwrap();

    assert_eq!(message, Message {
        junk: [82, 84, 80, 83]
//        submessages: vec![Submessage {
//            id: rtps::SubmessageId::DATA,
//            endianness: rtps::CdrEndianness::Little,
//            buf: rtps::common_types::ArcBuffer::from_vec(vec![])
//        }]
    });
}