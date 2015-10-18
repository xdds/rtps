use std::io::{ self, Write };

use message::types::*;

pub const STANDARD_HEADER : Header = Header {
  protocol_version: ProtocolVersion(2,2),
  vendor_id: VendorId(2),
  guid_prefix: GUIDPrefix(GUIDPREFIX_UNKNOWN)
};

const RTPS : &'static [u8; 4] = b"RTPS";
pub const GUIDPREFIX_UNKNOWN : [u8; 12] = [0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00];

pub struct Header {
  protocol_version: ProtocolVersion,
  vendor_id: VendorId,
  guid_prefix: GUIDPrefix
}

impl Header {
  pub fn serialize<W: Write>(&self, mut w: W) -> io::Result<()> {
    try!(w.write_all(RTPS));
    try!(self.protocol_version.serialize(&mut w));
    try!(self.vendor_id.serialize(&mut w));
    try!(self.guid_prefix.serialize(&mut w));
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::super::types::*;
  use super::super::header::*;

  #[test]
  fn test_serialize() {
    let h = Header{
      protocol_version: ProtocolVersion(2,2),
      vendor_id: VendorId(2),
      guid_prefix: GUIDPrefix(GUIDPREFIX_UNKNOWN)
    };
    let mut buf = vec![];
    h.serialize(&mut buf).unwrap();

    let expected = vec![
      // 'RTPS'
      82, 84, 80, 83,
      // protocol_version
      2, 2,
      // vendor_id
      2, 0,
      // guid_prefix
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];
    
    assert_eq!(buf, expected);
  }
}