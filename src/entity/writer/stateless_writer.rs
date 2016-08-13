/// RTPS Writer
/// Entity diagram found in formal-14-09-01 on manual page 71
/// Behavior statemachine found in formal-14-09-01

use super::super::super::common_types::*;
use super::super::{EntityTrait, EndpointTrait, WriterTrait, HistoryCache, HistoryCacheTrait};
use super::WriterInitArgs;

pub struct StatelessWriter {
    guid: Guid,
    unicast_locator_list: LocatorList,
    multicast_locator_list: LocatorList,
    reliability_level: ReliabilityKind,
    topic_kind: TopicKind,
    push_mode: bool,
    heartbeat_period: Duration,
    nack_response_delay: Duration,
    nack_suppression_duration: Duration,
    last_change_sequence_number: SequenceNumber,
    writer_cache: HistoryCache,
}

impl StatelessWriter {
    fn new(init_args: WriterInitArgs) -> Self {
        StatelessWriter {
            guid: init_args.guid,
            unicast_locator_list: init_args.unicast_locator_list,
            multicast_locator_list: init_args.multicast_locator_list,
            reliability_level: init_args.reliability_level,
            topic_kind: init_args.topic_kind,
            push_mode: init_args.push_mode,
            heartbeat_period: init_args.heartbeat_period,
            nack_response_delay: init_args.nack_response_delay,
            nack_suppression_duration: init_args.nack_suppression_duration,
            last_change_sequence_number: 0,
            writer_cache: HistoryCache::new(),
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
}

impl EntityTrait for StatelessWriter {
    fn guid(&self) -> Guid {
        self.guid
    }
}

impl EndpointTrait for StatelessWriter {
    fn reliability_level(&self) -> ReliabilityKind {
        self.reliability_level
    }

    fn topic_kind(&self) -> TopicKind {
        self.topic_kind
    }

    fn unicast_locator_list(&self) -> &LocatorList {
        &self.unicast_locator_list
    }

    fn multicast_locator_list(&self) -> &LocatorList {
        &self.multicast_locator_list
    }
}

impl WriterTrait for StatelessWriter {
    fn new_change(&mut self, kind: ChangeKind, handle: InstanceHandle, data: Vec<u8>) -> CacheChange {
        self.last_change_sequence_number += 1;

        CacheChange::new(kind, self.guid, handle, self.last_change_sequence_number, data)
    }
}