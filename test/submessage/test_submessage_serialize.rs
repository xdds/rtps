use rtps::cdr::{CdrSerializer};
use rtps::common_types as t;
use rtps::submessage::*;
use serde::ser::Serialize;

#[test]
fn test_submessage_serialize() {
    let submessage = Submessage {
        variant: SubmessageVariant::Data {
            reader_id: t::EntityId{ entity_key: [0,0,0], entity_kind: t::EntityKind::BuiltInReader },
            writer_id: t::EntityId{ entity_key: [0,0,0], entity_kind: t::EntityKind::BuiltInWriter },
            writer_sn: 5,
            serialized_payload: t::ArcBuffer::from_vec(vec![1,2,3,4, 11,12,13,14]),
        }
    };
    let buf : Vec<u8> = vec![];
    let mut serializer = CdrSerializer{
        endianness: t::Endianness::Big,
        write_handle: buf
    };
    submessage.serialize(&mut serializer).unwrap();
    let expected = vec![
        21, // submessage id = data
        0, // flags = null
        0, 0, 0, 255, // len = 255 (fake for now)

        0, 0, 0, 196,
        0, 0, 0, 195,
        0, 0, 0, 0, 0, 0, 0, 5,
        0, 0, 0, 8,
        1, 2, 3, 4, 11, 12, 13, 14];
    // [21, 0, 0, 0, 0, 255, 0, 0, 0, 196, 0, 0, 0, 195, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 8, 1, 2, 3, 4, 11, 12, 13, 14]
    // [21, 0, 0, 0, 0, 255, 196, 0, 0, 0, 195, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 8, 1, 2, 3, 4, 11, 12, 13, 14]
    assert_eq!(serializer.write_handle, expected);
}