use serde;
use serde::{ Deserialize };
use std::io::{ Read, Cursor };

use byteorder;
use byteorder::ByteOrder;

use rtps;
use rtps::cdr::CdrDeserializerError;
use rtps::cdr::CdrSeqVisitor;

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

    fn deserialize_u64<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_isize<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_i8<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_i16<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
    }

    fn deserialize_i32<V>(&mut self, _: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor {
        unimplemented!()
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

#[derive(Deserialize,Debug,PartialEq)]
pub struct Message {
    junk: [u8; 4],
    protocol_type: rtps::common_types::ProtocolVersion,
    vendor_id: rtps::common_types::VendorId,
    submessages: Vec<Submessage>
}

#[derive(Deserialize,Debug,PartialEq)]
pub struct Submessage {
    id: rtps::SubmessageId,
//    endianness: rtps::CdrEndianness,
//    buf: rtps::common_types::ArcBuffer
}

#[test]
fn bang() {
    let bytes = [
        82, 84, 80, 83, // "RTPS"
        20, 10, // Protocol Version
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
        junk: [82, 84, 80, 83],
        protocol_type: rtps::common_types::ProtocolVersion::VERSION_2_2,
        vendor_id: [86, 19],
        submessages: vec![Submessage {
            id: rtps::SubmessageId::DATA
        }],
    });
}