use serde::{ Deserialize };
use std::io::{ Cursor };

use rtps;

#[test]
fn deserialize_message() {
    let bytes = [
        82, 84, 80, 83, // "RTPS"
        20, 10, // Protocol Version
        86, 19, // Vendor id
        0, 0, 0, 1, // Submessage count
        21, 1, // Submessage 0 message id, endianness flag
        0, 0, 0, 4, // Submessage 0 len
        1, 2, 3, 4  // Submessage 0 data
    ];
    let mut cursor = Cursor::new(bytes);

    {
        let mut de = rtps::cdr::CdrDeserializer::new(&mut cursor);
        let message: rtps::Message = Deserialize::deserialize(&mut de).unwrap();

        assert_eq!(message, rtps::Message {
            junk: [82, 84, 80, 83],
            protocol_type: rtps::common_types::ProtocolVersion::VERSION_2_2,
            vendor_id: [86, 19],
            submessages: vec![rtps::Submessage {
                id: rtps::SubmessageId::DATA,
                endianness: rtps::common_types::Endianness::Little,
                buf: rtps::common_types::ArcBuffer::from_vec(vec![1,2,3,4])
            }],
        });
    }
    assert_eq!(cursor.position() as usize, bytes.len());
}