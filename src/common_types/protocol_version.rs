use std::default::Default;
use serde;

#[allow(non_camel_case_types)]
#[derive(Clone,Copy,PartialEq,Debug)]
pub enum ProtocolVersion {
    VERSION_2_2,
    UNKNOWN
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        ProtocolVersion::VERSION_2_2
    }
}

impl serde::Deserialize for ProtocolVersion {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
        let id_bytes : [u8; 2] = try!(serde::Deserialize::deserialize(deserializer));
        if id_bytes == [20,10] {
            Ok(ProtocolVersion::VERSION_2_2)
        } else {
            Ok(ProtocolVersion::UNKNOWN)
        }
    }
}

impl serde::Serialize for ProtocolVersion {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
        let val : [u8; 2] = match *self {
            ProtocolVersion::VERSION_2_2 => [20,10],
            ProtocolVersion::UNKNOWN => return Err(serde::ser::Error::custom("unknown protocol version"))
        };

        val.serialize(serializer)
    }
}
