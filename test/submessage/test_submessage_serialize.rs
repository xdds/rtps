use rtps::cdr::{CdrSerializer,CdrEndianness};
use rtps::common_types::*;
use rtps::submessage::*;
use serde::ser::Serialize;

#[test]
fn test_serialize() {
    let submessage = Submessage {
        id: SubmessageId::DATA,
        endianness: CdrEndianness::Little,
        buf: ArcBuffer::from_vec(vec![1,2,3,4])
    };
    let buf : Vec<u8> = vec![];
    let mut serializer = CdrSerializer{
        endianness: CdrEndianness::Big,
        write_handle: buf
    };
    submessage.serialize(&mut serializer).unwrap();
    let expected = vec![0x15, 1,
        0, 0, 0, 4,
        1, 2, 3, 4
    ];
    assert_eq!(serializer.write_handle, expected);
}