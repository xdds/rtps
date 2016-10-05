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
        TC {
            b: &[
                0x09, 1, // Submessage 0 message id, endianness flag
                0, 0, 0, 8, // Submessage 0 len
                0, 0, 1, 0, // Submessage 0 seconds
                0, 0, 0, 1, // Submessage 0 fraction
            ],
            e: rtps::SubmessageVariant::InfoTimestamp(t::Timestamp { seconds: 256, fraction: 1 })
        },
        TC {
            b: &[
                0x0c, 1, // Submessage 0 message id, endianness flag
                0, 0, 0, 16, // Submessage 0 len
                20, 10, 1, 0,  // protocol version, vendor id
                0xFF, 0xFF, 0xFF, 0xFF, // guid first 4
                0x00, 0x00, 0x00, 0x00, // guid second 4
                0xDD, 0xDD, 0xDD, 0xDD, // guid third 4
            ],
            e: rtps::SubmessageVariant::InfoSource {
                guid_prefix: [0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xDD, 0xDD, 0xDD, 0xDD],
                protocol_version: t::ProtocolVersion::VERSION_2_2,
                vendor_id: [1, 0],
            }
//                (t::Timestamp { seconds: 4, fraction: 256 })
        }
    ];

    for &TC{ref b, ref e} in test_cases.iter() {
        let mut cursor = Cursor::new(*b);
        {
            let mut de = rtps::cdr::CdrDeserializer::new(&mut cursor);
            let message: rtps::SubmessageV2 = Deserialize::deserialize(&mut de).unwrap();
            assert_eq!(message.variant, *e);
        }
        assert_eq!(cursor.position() as usize, (*b).len());
    }
}