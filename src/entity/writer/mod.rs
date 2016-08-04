/// RTPS Writer
/// Entity diagram found in formal-14-09-01 on manual page 71
/// Behavior statemachine found in formal-14-09-01

use super::super::common_types::*;
use super::{ EntityTrait, EndpointTrait, HistoryCache };

pub struct Writer {
    pub guid: Guid,
    pub unicast_locator_list: LocatorList,
    pub multicast_locator_list: LocatorList,
    pub reliability_level: ReliabilityKind,
    pub topic_kind: TopicKind,
    pub push_mode: bool,
    pub heartbeat_period: Duration,
    pub nack_response_delay: Duration,
    pub nack_suppression_duration: Duration,
    pub last_change_sequence_number: SequenceNumber,
    pub writer_cache: HistoryCache,
    _secret: (),
}

impl Writer {
    fn new(guid: Guid, unicast_locator_list: LocatorList, multicast_locator_list: LocatorList,
           reliability_level: ReliabilityKind, topic_kind: TopicKind, push_mode: bool,
           heartbeat_period: Duration, nack_response_delay: Duration,
           nack_suppression_duration: Duration) -> Self {
        Writer {
            guid: guid,
            unicast_locator_list: unicast_locator_list,
            multicast_locator_list: multicast_locator_list,
            reliability_level: reliability_level,
            topic_kind: topic_kind,
            push_mode: push_mode,
            heartbeat_period: heartbeat_period,
            nack_response_delay: nack_response_delay,
            nack_suppression_duration: nack_suppression_duration,
            last_change_sequence_number: 0,
            writer_cache: HistoryCache::new(),
            _secret: ()
        }
    }

    fn push_mode(&self) -> bool {
        self.push_mode
    }

    fn heartbeat_period(&self) -> Duration {
        self.heartbeat_period
    }

    fn nack_response_delay(&self) -> Duration {
        self.nack_response_delay
    }

    fn nack_suppression_duration(&self) -> Duration {
        self.nack_response_delay
    }

    fn new_change(&self, change: ChangeKind, handle: InstanceHandle, data: Vec<u8>) {
        self.writer_cache.add(change, handle, data)
    }
}

impl EntityTrait for Writer {
    fn guid(&self) -> Guid {
        self.guid
    }
}

impl EndpointTrait for Writer {
    fn topic_kind(&self) -> TopicKind {
        self.topic_kind
    }
    fn reliability_level(&self) -> ReliabilityKind {
        self.reliability_level
    }
    fn unicast_locator_list(&self) -> &LocatorList {
        &self.unicast_locator_list
    }
    fn multicast_locator_list(&self) -> &LocatorList {
        &self.multicast_locator_list
    }
}