use rtps::cdr::{CdrSerializer};
use rtps::common_types::*;
use rtps::message::*;
use rtps::submessage::*;
use serde::ser::Serialize;

#[test]
fn test_serialize() {
    let submessage = Submessage {
        id: SubmessageId::DATA,
        endianness: Endianness::Little,
        buf: ArcBuffer::from_vec(vec![1,2,3,4])
    };
    let message = Message::new(vec![submessage]);

    let buf : Vec<u8> = vec![];
    let mut serializer = CdrSerializer{
        endianness: Endianness::Big,
        write_handle: buf
    };
    message.serialize(&mut serializer).unwrap();
    let expected = vec![
            82, 84, 80, 83, // RTPS
            20, 10, // Protocol Type
            86, 19, // Vendor id
            0, 0, 0, 1, // Submessage count
            21, 1,
            0, 0, 0, 4,
            1, 2, 3, 4
        ];
    assert_eq!(serializer.write_handle, expected);
}