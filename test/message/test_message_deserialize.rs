use serde::{ Deserialize };
use std::io::{ Cursor };

use rtps;
use rtps::common_types as t;

#[test]
fn deserialize_message() {
    let bytes = [
        82, 84, 80, 83, // "RTPS"
        20, 10, // Protocol Version
        86, 19, // Vendor id
        0, 0, 0, 1, // Submessage count

        0x15, 1, // Submessage 0 message id, endianness flag
        0xDD, 0xEE, 0xAA, 0xDD, // Submessage 0 len

        0, 0, 0, 0xc4, // reader_id
        0, 0, 0, 0xc3, // writer_id
        0, 0, 0, 0, // writer_sn first 4
        0, 0, 0, 5, // writer_sn second 4

        0, 0, 0, 8, // serialized_payload len
        1, 2, 3, 4, // payload first 4
        11, 12, 13, 14, // payload second 4
    ];
    let mut cursor = Cursor::new(&bytes[..]);

    {
        let mut de = rtps::cdr::CdrDeserializer::new(&mut cursor);
        let message: rtps::Message = Deserialize::deserialize(&mut de).unwrap();

        assert_eq!(message, rtps::Message {
            junk: [82, 84, 80, 83],
            protocol_type: rtps::common_types::ProtocolVersion::VERSION_2_2,
            vendor_id: [86, 19],
            submessages: vec![rtps::Submessage{
                    variant: rtps::SubmessageVariant::Data {
                        reader_id: t::EntityId{ entity_key: [0,0,0], entity_kind: t::EntityKind::BuiltInReader },
                        writer_id: t::EntityId{ entity_key: [0,0,0], entity_kind: t::EntityKind::BuiltInWriter },
                        writer_sn: 5,
                        serialized_payload: t::ArcBuffer::from_vec(vec![1,2,3,4, 11,12,13,14]),
                    }
                }
            ]
        });
    }
    assert_eq!(cursor.position() as usize, bytes.len());
}