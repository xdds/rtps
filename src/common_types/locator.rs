use std::io;
use std::net::UdpSocket;

pub type LocatorAddress = [u8; 16];
pub type LocatorPort = u32;

#[allow(non_camel_case_types)]
pub enum Locator {
    INVALID,
    KIND_INVALID,
    KIND_RESERVED,
    KIND_UDPv4(LocatorAddress, LocatorPort),
    KIND_UDPv6(LocatorAddress, LocatorPort),
    ADDRESS_INVALID,
    PORT_INVALID,

    BUF(Vec<u8>)
}

impl Locator {
    pub fn write(&self, buf: &[u8]) -> io::Result<()> {
        let conn = try!(UdpSocket::bind("0.0.0.0:0"));
        try!(conn.connect("127.0.0.1:9093"));
        match conn.send(buf) {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }
}

pub type LocatorList = Vec<Locator>;