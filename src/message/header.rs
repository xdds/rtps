use std::io::{self, Write};

use message::types::*;

pub const STANDARD_HEADER: Header = Header {
    protocol_version: ProtocolVersion(2, 2),
    vendor_id: VendorId(2),
    guid_prefix: GUIDPrefix(GUIDPREFIX_UNKNOWN)
};

const RTPS: &'static [u8; 4] = b"RTPS";
pub const GUIDPREFIX_UNKNOWN: [u8; 12] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

pub struct Header {
    pub protocol_version: ProtocolVersion,
    pub vendor_id: VendorId,
    pub guid_prefix: GUIDPrefix
}

impl Header {
    pub fn serialize_old<W: Write>(&self, mut w: W) -> io::Result<()> {
        try!(w.write_all(RTPS));
        try!(self.protocol_version.serialize(&mut w));
        try!(self.vendor_id.serialize(&mut w));
        try!(self.guid_prefix.serialize(&mut w));
        Ok(())
    }
}
