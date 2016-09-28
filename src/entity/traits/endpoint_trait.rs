use std::default::Default;
use super::super::super::common_types::*;
use super::super::traits::*;

pub trait EndpointTrait : EntityTrait {
    fn topic_kind(&self) -> TopicKind;

    fn reliability_level(&self) -> ReliabilityKind {
        Default::default()
    }

    fn unicast_locator_list(&self) -> &LocatorList;

    fn multicast_locator_list(&self) -> &LocatorList;
}