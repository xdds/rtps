use serde::{Deserialize};
use std::io::{Cursor};

use rtps;
use rtps::common_types as t;

#[test]
fn deserialize_submessage() {
    struct TC<'a> {
        b: &'a [u8],
        e: rtps::SubmessageVariant
    };

    let test_cases = [
        //
        TC {
            b: &[
                0x09, 1, // Submessage 0 message id, endianness flag
                0xDD, 0xEE, 0xAA, 0xDD, // Submessage 0 len
                0, 0, 1, 0, // Submessage 0 seconds
                0, 0, 0, 1, // Submessage 0 fraction
            ],
            e: rtps::SubmessageVariant::InfoTimestamp(t::Timestamp { seconds: 256, fraction: 1 })
        },
        TC {
            b: &[
                0x0c, 1, // Submessage 0 message id, endianness flag
                0xDD, 0xEE, 0xAA, 0xDD, // Submessage 0 len
                20, 10, 1, 0, // protocol version, vendor id
                0xFF, 0xFF, 0xFF, 0xFF, // guid first 4
                0x00, 0x00, 0x00, 0x00, // guid second 4
                0xDD, 0xDD, 0xDD, 0xDD, // guid third 4
            ],
            e: rtps::SubmessageVariant::InfoSource {
                guid_prefix: [0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xDD, 0xDD, 0xDD, 0xDD],
                protocol_version: t::ProtocolVersion::VERSION_2_2,
                vendor_id: [1, 0],
            }
        },
        TC {
            b: &[
                0x0f, 1, // Submessage 0 message id, endianness flag
                0xDD, 0xEE, 0xAA, 0xDD, // Submessage 0 len
                0, 0, 0, 1, // number of unicast locators
                0, 0, 0, 1, // locator kind
                0, 0, 0, 0, // locator port
                0, 0, 0, 0, // locator first 4
                0, 0, 0, 0, // locator second 4
                0, 0, 0, 0, // locator third 4
                0xFF, 0xFF, 0xFF, 0xFF, // locator fourth 4
            ],
            e: rtps::SubmessageVariant::InfoReply {
                unicast_locator_list: vec![t::Locator::KIND_UDPv4(0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255])],
            }
        },
        TC {
            b: &[
                0x0e, 1, // Submessage 0 message id, endianness flag
                0xDD, 0xEE, 0xAA, 0xDD, // Submessage 0 len
                1, 2, 3, 4, // guid prefix first four
                5, 6, 7, 8, // guid prefix second four
                9, 10, 11, 12, // guid prefix locator port
            ],
            e: rtps::SubmessageVariant::InfoDestination([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])
        },
        TC {
            b: &[
                0x12, 1, // Submessage 0 message id, endianness flag
                0xDD, 0xEE, 0xAA, 0xDD, // Submessage 0 len
                0, 0, 0, 0xc4, // reader_id
                0, 0, 0, 0xc3, // writer_id
                0, 0, 0, 0, // writer_sn first 4
                0, 0, 0, 5, // writer_sn second 4

                0, 0, 0, 100, // fragment_number base
                4, 3, 2, 1, // fragment_number bitset part 1
                4, 3, 2, 1, // fragment_number bitset part 2
                4, 3, 2, 1, // fragment_number bitset part 3
                4, 3, 2, 1, // fragment_number bitset part 4
                4, 3, 2, 1, // fragment_number bitset part 5
                4, 3, 2, 1, // fragment_number bitset part 6
                4, 3, 2, 1, // fragment_number bitset part 7
                4, 3, 2, 1, // fragment_number bitset part 8

                0, 0, 0, 5, // count
            ],
            e: rtps::SubmessageVariant::NackFrag {
                reader_id: t::EntityId{ entity_key: [0,0,0], entity_kind: t::EntityKind::BuiltInReader },
                writer_id: t::EntityId{ entity_key: [0,0,0], entity_kind: t::EntityKind::BuiltInWriter },
                writer_sn: 5,
                fragment_number_state: t::FragmentNumberSet{ base: 100, set: [4, 3, 2, 1, 4, 3, 2, 1, 4, 3, 2, 1, 4, 3, 2, 1, 4, 3, 2, 1, 4, 3, 2, 1, 4, 3, 2, 1, 4, 3, 2, 1] },
                count: 5,
            }
        }
    ];

    for &TC { ref b, ref e } in test_cases.iter() {
        let mut cursor = Cursor::new(*b);
        {
            let mut de = rtps::cdr::CdrDeserializer::new(&mut cursor);
            let message: rtps::SubmessageV2 = Deserialize::deserialize(&mut de).unwrap();
            assert_eq!(message.variant, *e);
        }
        assert_eq!(cursor.position() as usize, (*b).len());
    }
}