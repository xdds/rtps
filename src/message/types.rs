use std::io::{ self, Write };
use byteorder::{ LittleEndian, WriteBytesExt};

pub struct GUIDPrefix(pub [u8; 12]);
impl GUIDPrefix {
  pub fn serialize<W: Write>(&self, mut w: &mut W) -> io::Result<()> {
    try!(w.write_all(&self.0[..]));
    Ok(())
  }
}

pub struct ProtocolVersion(pub u8, pub u8);
impl ProtocolVersion {
  pub fn serialize<W: Write>(&self, mut w: &mut W) -> io::Result<()> {
    let bytes = &[self.0, self.1][..];
    try!(w.write(bytes));
    Ok(())
  }
}

pub struct VendorId(pub u16);
impl VendorId {
  pub fn serialize<W: Write>(&self, mut w: &mut W) -> io::Result<()> {
    try!(w.write_u16::<LittleEndian>(self.0));
    Ok(())
  }
}