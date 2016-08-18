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

pub type LocatorList = Vec<Locator>;