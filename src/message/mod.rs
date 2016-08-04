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

#[cfg(test)]
mod tests {
    use Message;
    use submessage::{Submessage, SubmessageId};
    use super::super::cdr::{CdrSerializer, CdrEndianness};

    use serde::ser::Serialize;

    //  use submessage::*;

    #[test]
    fn serializes(){
        let buf : Vec<u8> = vec![];
        let m = Message {
            submessages: vec![
                Submessage(SubmessageId::DATA,
                    CdrEndianness::Little,
                    vec![1,2,3,4]
                )
            ]
        };
        let mut serializer = CdrSerializer{
            endianness: CdrEndianness::Big,
            write_handle: buf
        };

        m.serialize(&mut serializer).unwrap();

        let expected = vec![
            82, 84, 80, 83, // RTPS
            20, 10, // Protocol Type
            86, 19, // Vendor id
            0, 0, 0, 1, // Submessage count
            21, 1,
            0, 0, 0, 4,
            1, 2, 3, 4
        ];

        assert_eq!(serializer.write_handle, expected);
    }

}