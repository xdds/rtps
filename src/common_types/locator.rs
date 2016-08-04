pub struct Locator {
    pub kind: i32,
    pub port: u32,
    pub address: [u8; 16],
}

pub type LocatorList = Vec<Locator>;