use std::default::Default;
use super::super::super::common_types::*;

#[derive(Default)]
pub struct ReaderInitArgs {
    pub guid: Guid,
    pub unicast_locator_list: LocatorList,
    pub multicast_locator_list: LocatorList
}

impl ReaderInitArgs {
    pub fn new() -> Self {
        ReaderInitArgs {
            guid: Default::default(),
            unicast_locator_list: Default::default(),
            multicast_locator_list: Default::default()
        }
    }
}