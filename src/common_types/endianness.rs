use serde;

#[derive(Debug,PartialEq)]
pub enum Endianness {
    Little,
    Big
}

impl serde::Deserialize for Endianness {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
        let byte : u8 = try!(serde::Deserialize::deserialize(deserializer));
        if byte == 1 {
            Ok(Endianness::Little)
        } else {
            Ok(Endianness::Big)
        }
    }
}