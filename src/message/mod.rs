pub mod types;
pub mod header;
pub mod submessage;

use self::submessage::*;

pub struct Message {
  submessages: Vec<Submessage>
}

#[cfg(test)]
mod tests {
//  use Message;
//  use submessage::*;

//  #[test]
//  fn serializes(){
//    let m = Message{
//      submessages: vec![
//        Submessage(DATA, vec![0,1,2,3]),
//        Submessage(DATA, vec![4,5,6,7])
//      ]
//    };
//    let mut buf = vec![];
//    m.serialize_old(&mut buf).unwrap();
//
//    let expected = vec![
//      // 'RTPS'
//      82, 84, 80, 83,
//      // protocol_version
//      2, 2,
//      // vendor_id
//      2, 0,
//      // guid_prefix
//      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//      // submessage_0 index
//      0,
//      // submessage_0 flags
//      0x15,
//      // submessage_0 len
//      4, 0,
//      // submessage_0 body
//      0, 1, 2, 3,
//      // subessage_1 index
//      1,
//      // submessage_1 flags
//      0x15,
//      // submessage_1 len
//      4, 0,
//      // submessage_1 body
//      4, 5, 6, 7
//    ];
//
//    assert_eq!(buf, expected);
//  }

}