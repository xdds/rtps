pub mod header;

use serde::ser::{ Serialize, Serializer };
use byteorder::{ ByteOrder, LittleEndian };

use common_types::*;
use submessage::*;

#[derive(Deserialize,Debug,PartialEq)]
pub struct Message {
    pub junk: [u8; 4],
    pub protocol_type: ProtocolVersion,
    pub vendor_id: VendorId,
    pub submessages: Vec<Submessage>
}

const VERSION_BYTES : [u8; 2] = [10, 20];
const VENDOR_ID : [u8; 2] = [19, 86];

impl Message {
    pub fn new(submessages: Vec<Submessage>) -> Self {
        Message {
            junk: [82, 84, 80, 83],
            protocol_type: ProtocolVersion::VERSION_2_2,
            vendor_id: [86, 19],
            submessages
        }
    }


}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        try!(serializer.serialize_str("RTPS"));

        try!(serializer.serialize_u16(LittleEndian::read_u16(&VERSION_BYTES)));
        try!(serializer.serialize_u16(LittleEndian::read_u16(&VENDOR_ID)));

        let mut state = try!( serializer.serialize_seq( Some(self.submessages.len()) ) );
        for subm in &self.submessages {
            try!(serializer.serialize_seq_elt(&mut state, subm));
        }
        serializer.serialize_seq_end(state)
    }

}