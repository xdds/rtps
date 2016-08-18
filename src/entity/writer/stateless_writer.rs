/// RTPS Writer
/// Entity diagram found in formal-14-09-01 on manual page 71
/// Behavior statemachine found in formal-14-09-01

use super::super::super::common_types::*;
use super::super::{EntityTrait, EndpointTrait, WriterTrait, HistoryCache, HistoryCacheTrait};
use super::WriterInitArgs;
use super::super::super::message::Heartbeat;

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

    heartbeat_count: u32,
}

impl StatelessWriter {
    pub fn new(init_args: WriterInitArgs) -> Self {
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
            writer_cache: HistoryCache::new(),

            last_change_sequence_number: 0,
            heartbeat_count: 0
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

    pub fn heartbeat(&mut self, reader_id: EntityId) -> Heartbeat {
        let max = self.writer_cache.get_seq_num_max().unwrap_or(0);
        let min = self.writer_cache.get_seq_num_min().unwrap_or(0);

        let heartbeat = Heartbeat {
            is_key: false,
            reader_id: reader_id,
            writer_id: self.guid.entity_id,

            first_sn: min,
            last_sn: max,

            count: self.heartbeat_count
        };

        self.heartbeat_count += 1;

        heartbeat
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
    fn new_change(&mut self, kind: ChangeKind, handle: InstanceHandle, data: RcBuffer) -> CacheChange {
        self.last_change_sequence_number += 1;

        let change = CacheChange::new(kind, self.guid, handle, self.last_change_sequence_number, data);
        self.writer_cache.add_change(&change).unwrap();
        change
    }
}