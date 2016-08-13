use std::default::Default;
use super::super::super::common_types::*;

pub trait ParticipantTrait {
    fn protocol_version(&self) -> ProtocolVersion {
        Default::default()
    }

    fn vendor_id(&self) -> VendorId {
        [0,0]
    }

    fn default_unicast_locator_list(&self) -> &LocatorList;
    fn default_multicast_locator_list(&self) -> &LocatorList;
}