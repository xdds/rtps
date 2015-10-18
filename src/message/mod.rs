pub mod types;
pub mod header;
pub mod submessage;

use self::submessage::*;

use std::io::{ self, Write};

pub struct Message {
  submessages: Vec<Submessage>
}

impl Message {
  fn serialize<W: Write>(&self, mut w: &mut W) -> io::Result<()> {
    let h = header::STANDARD_HEADER;
    try!(h.serialize(&mut w));

    let mut counter : [u8; 1] = [0];
    for submessage in &self.submessages {
      try!(w.write(&counter[..]));
      try!(submessage.serialize(&mut w));

      counter[0] = counter[0] + 1;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use Message;
  use submessage::*;

  #[test]
  fn serializes(){
    let m = Message{
      submessages: vec![
        Submessage(DATA, vec![0,1,2,3]),
        Submessage(DATA, vec![4,5,6,7])
      ]
    };
    let mut buf = vec![];
    m.serialize(&mut buf).unwrap();

    let expected = vec![
      // 'RTPS'
      82, 84, 80, 83,
      // protocol_version
      2, 2,
      // vendor_id
      2, 0,
      // guid_prefix
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      // submessage_0 index
      0,
      // submessage_0 body
      0, 1, 2, 3,
      // subessage_1 index
      1,
      // submessage_1 body
      4, 5, 6, 7
    ];

    assert_eq!(buf, expected);
  }

}