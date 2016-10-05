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
                0x0c, 1, // Submessage 0 message id, endianness flag
                0, 0, 0, 4, // Submessage 0 len
                0, 0, 1, 0  // Submessage 0 data
            ],
            e: rtps::SubmessageVariant::InfoTimestamp(t::Timestamp { seconds: 4, fraction: 256 })
        },
        TC {
            b: &[
                0x09, 1, // Submessage 0 message id, endianness flag
                0, 0, 0, 4, // Submessage 0 len
                0, 0, 1, 0  // Submessage 0 data
            ],
            e: rtps::SubmessageVariant::InfoTimestamp(t::Timestamp { seconds: 4, fraction: 256 })
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