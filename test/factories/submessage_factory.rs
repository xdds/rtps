use rtps::*;
use rtps::common_types::*;
use factories::*;

impl Create for Submessage {
    fn create_variant(id: &str) -> Self {
        let variant = match id {
            "InfoDestination" => {
                SubmessageVariant::InfoDestination([0,0,0,0, 0,0,0,0, 0,0,0,0])
            }
            "InfoReply" => {
                SubmessageVariant::InfoReply{
                    unicast_locator_list: Create::create()
                }
            }
            "InfoTimestamp" => {
                SubmessageVariant::InfoTimestamp(Timestamp {
                    seconds: 10,
                    fraction: 100
                })
            }
            "InfoSource" => {
                SubmessageVariant::InfoSource {
                    protocol_version: ProtocolVersion::VERSION_2_2,
                    vendor_id: [10, 20],
                    guid_prefix: Guid::new().guid_prefix,
                }
            }

            "AckNack" => {
                SubmessageVariant::AckNack {
                    reader_id: EntityId::builtin_unknown(),
                    writer_id: EntityId::builtin_unknown(),
                    reader_sn_state: SequenceNumberSet{base: 100, bitmap: [0; 32]},
                    count: 100,
                }
            }
            "Data" => {
                SubmessageVariant::Data {
                    reader_id: EntityId::builtin_unknown(),
                    writer_id: EntityId::builtin_unknown(),
                    writer_sn: 100/*, inline_qos: Option<InlineQOS> */,
                    serialized_payload: ArcBuffer::from_vec(vec![1,2,3])
                }
            }
            "DataFrag" => {
                SubmessageVariant::DataFrag {
                    reader_id: EntityId::builtin_unknown(),
                    writer_id: EntityId::builtin_unknown(),
                    writer_sn: 100/*, inline_qos: Option<InlineQOS> */,
                    serialized_payload: ArcBuffer::from_vec(vec![1,2,3]),
                    data_size: 1000,
                    fragment_size: 100,
                    fragment_start_num: 2,
                    fragments_in_submessage: 1
                }
            }
            "Gap" => {
                SubmessageVariant::Gap {
                    gap_start: 100,
                    gap_list: SequenceNumberSet{ base: 777, bitmap: [1; 32]},
                    reader_id: EntityId::builtin_unknown(),
                    writer_id: EntityId::builtin_unknown()
                }
            }
            "Heartbeat" => {
                SubmessageVariant::Heartbeat {
                    writer_id: EntityId::builtin_unknown(),
                    reader_id: EntityId::builtin_unknown(),
                    first_sn: 1,
                    last_sn: 2,
                    count: 206,
                }
            }
            "HeartbeatFrag" => {
                SubmessageVariant::HeartbeatFrag {
                    reader_id: EntityId{ entity_key: [0,0,0], entity_kind: EntityKind::BuiltInReader },
                    writer_id: EntityId{ entity_key: [0,0,0], entity_kind: EntityKind::BuiltInWriter },
                    writer_sn: 5,
                    last_fragment_number: 10,
                    count: 5,
                }
            }
            "NackFrag" => {
                SubmessageVariant::NackFrag {
                    reader_id: EntityId{ entity_key: [0,0,0], entity_kind: EntityKind::BuiltInReader },
                    writer_id: EntityId{ entity_key: [0,0,0], entity_kind: EntityKind::BuiltInWriter },
                    writer_sn: 5,
                    fragment_number_state: FragmentNumberSet{ base: 100, set: [4, 3, 2, 1, 4, 3, 2, 1, 4, 3, 2, 1, 4, 3, 2, 1, 4, 3, 2, 1, 4, 3, 2, 1, 4, 3, 2, 1, 4, 3, 2, 1] },
                    count: 5,
                }
            }
            "Pad" => {
                SubmessageVariant::Pad {

                }
            }
            _ => {
                panic!("factory doesn't know {:?}", id)
            }
        };

        Submessage { variant: variant }
    }
}
