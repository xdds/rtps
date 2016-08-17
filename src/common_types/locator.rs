pub type LocatorAddress = [u8; 16];
pub type LocatorPort = u32;

#[allow(non_camel_case_types)]
pub enum Locator {
    LOCATOR_INVALID,
    LOCATOR_KIND_INVALID,
    LOCATOR_KIND_RESERVED,
    LOCATOR_KIND_UDPv4(LocatorAddress, LocatorPort),
    LOCATOR_KIND_UDPv6(LocatorAddress, LocatorPort),
    LOCATOR_ADDRESS_INVALID,
    LOCATOR_PORT_INVALID
}

pub type LocatorList = Vec<Locator>;