use rtps::cdr::{CdrSerializer};
use rtps::common_types as t;
use rtps::message::*;
use rtps::submessage::*;
use serde::ser::Serialize;

#[test]
fn test_serialize() {
    let submessage = Submessage {
        variant: SubmessageVariant::Data {
            reader_id: t::EntityId{ entity_key: [0,0,0], entity_kind: t::EntityKind::BuiltInReader },
            writer_id: t::EntityId{ entity_key: [0,0,0], entity_kind: t::EntityKind::BuiltInWriter },
            writer_sn: 5,
            serialized_payload: t::ArcBuffer::from_vec(vec![1,2,3,4])
        }
    };
    let message = Message::new(vec![submessage]);

    let buf : Vec<u8> = vec![];
    let mut serializer = CdrSerializer{
        endianness: t::Endianness::Big,
        write_handle: buf
    };
    message.serialize(&mut serializer).unwrap();
    let expected = vec![
            82, 84, 80, 83, // RTPS
            20, 10, // Protocol Type
            86, 19, // Vendor id
            0, 0, 0, 1, // Submessage count
            21, 0, 0, 0,
            3, 0, 0, 0,
            196, 0, 0, 0,
            3, 0, 0, 0,
            195, 0, 0, 0,
            0, 0, 0, 0, 5,
            0, 0, 0, 4,
            1, 2, 3, 4
        ];
    assert_eq!(serializer.write_handle, expected);
}