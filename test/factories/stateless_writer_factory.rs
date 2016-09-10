use rtps::*;
use rtps::common_types::*;
use factories::Create;

impl Create for StatelessWriter {
    fn create() -> Self {
        StatelessWriter::new(WriterInitArgs{
            guid: Guid::new(),
            unicast_locator_list: LocatorList::new(),
            multicast_locator_list: LocatorList::new(),
            reliability_level: ReliabilityKind::BEST_EFFORT,
            topic_kind: TopicKind::NO_KEY,
            push_mode: false,
            heartbeat_period: Duration::new(10,0),
            nack_response_delay: Duration::new(10,0),
            nack_suppression_duration: Duration::new(10,0),
        })
    }
}