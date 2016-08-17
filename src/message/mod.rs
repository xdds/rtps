pub mod types;
pub mod header;

use serde::ser::{ Serialize, Serializer };
use serde::ser::impls::SeqIteratorVisitor;
use byteorder::{ ByteOrder, LittleEndian };

use submessage::*;

pub struct Message {
    submessages: Vec<Submessage>
}

const VERSION_BYTES : [u8; 2] = [10, 20];
const VENDOR_ID : [u8; 2] = [19, 86];


impl Serialize for Message {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        try!(serializer.serialize_str("RTPS"));

        try!(serializer.serialize_u16(LittleEndian::read_u16(&VERSION_BYTES)));
        try!(serializer.serialize_u16(LittleEndian::read_u16(&VENDOR_ID)));


        let visitor = SeqIteratorVisitor::new(self.submessages.iter(), Some(self.submessages.len()));
        serializer.serialize_seq(visitor)
    }
}