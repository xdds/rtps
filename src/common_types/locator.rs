use std::io;
use std::net::UdpSocket;

use serde;

pub type LocatorAddress = [u8; 16];
pub type LocatorPort = u32;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum Locator {
    INVALID,
    KIND_INVALID,
    KIND_RESERVED,
    KIND_UDPv4(LocatorPort, LocatorAddress),
    KIND_UDPv6(LocatorPort, LocatorAddress),
    ADDRESS_INVALID,
    PORT_INVALID,

    BUF(Vec<u8>)
}

impl serde::Deserialize for Locator {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
        let kind = try!(serde::Deserialize::deserialize(deserializer));

        match kind {
            LocatorKind::UDPv4 => {
                Ok(Locator::KIND_UDPv4(
                    try!(serde::Deserialize::deserialize(deserializer)),
                    try!(serde::Deserialize::deserialize(deserializer))
                ))
            },
            _ => {
                Err(serde::Error::custom(format!("we don't support {:?}", kind)))
            }
        }
    }
}

impl serde::Serialize for Locator {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
        match *self {
            Locator::KIND_UDPv4(port, locator) => {
                try!((LocatorKind::UDPv4).serialize(serializer));
                try!(port.serialize(serializer));
                try!(locator.serialize(serializer));
                Ok(())
            },
            _ => {
                unimplemented!()
            }
        }
    }
}

impl Locator {
    pub fn to_str(& self) -> String {
        match *self {
            Locator::KIND_UDPv4(port, addr) => format!("{}.{}.{}.{}:{}", addr[12], addr[13], addr[14], addr[15], port),
            _ => "no se".to_owned()

        }
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<()> {
        let conn = try!(UdpSocket::bind("0.0.0.0:0"));
        try!(conn.connect(&self.to_str()[..]));
        match conn.send(buf) {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
}

#[derive(PartialEq, Debug)]
enum LocatorKind {
    INVALID,
    // = -1
    //    ADDRESS_INVALID, // {0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0}
    //    PORT_INVALID, // 0
    RESERVED,
    // 0
    UDPv4,
    // 1
    UDPv6,
    // 2

    // #define LOCATOR_PORT_INVALID 0
    // #define LOCATOR_KIND_RESERVED 0
    // #define LOCATOR_KIND_UDPv4 1
    // #define LOCATOR_KIND_UDPv6 2
}

impl serde::Deserialize for LocatorKind {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
        let kind: i32 = try!(serde::Deserialize::deserialize(deserializer));
        match kind {
            0 => Ok(LocatorKind::RESERVED),
            1 => Ok(LocatorKind::UDPv4),
            2 => Ok(LocatorKind::UDPv6),
            _ => {
                Ok(LocatorKind::INVALID)
            },
        }
    }
}

impl serde::Serialize for LocatorKind {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
        let val = match *self {
            LocatorKind::RESERVED => 0,
            LocatorKind::UDPv4 => 1,
            LocatorKind::UDPv6 => 2,
            _ => -1
        };

        serializer.serialize_i32(val)
    }
}

pub type LocatorList = Vec<Locator>;