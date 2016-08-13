use std::default::Default;
use super::super::super::common_types::*;

#[derive(Default)]
pub struct WriterInitArgs {
    pub guid: Guid,
    pub unicast_locator_list: LocatorList,
    pub multicast_locator_list: LocatorList,
    pub reliability_level: ReliabilityKind,
    pub topic_kind: TopicKind,
    pub push_mode: bool,
    pub heartbeat_period: Duration,
    pub nack_response_delay: Duration,
    pub nack_suppression_duration: Duration
}

impl WriterInitArgs {
    pub fn new() -> Self {
        WriterInitArgs {
            guid: Default::default(),
            unicast_locator_list: Default::default(),
            multicast_locator_list: Default::default(),
            reliability_level: Default::default(),
            topic_kind: Default::default(),
            push_mode: Default::default(),
            heartbeat_period: Duration::new(1,0),
            nack_response_delay: Duration::new(1,0),
            nack_suppression_duration: Duration::new(1,0),
        }
    }
}