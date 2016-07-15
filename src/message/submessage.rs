use super::super::cdr::ser::CdrEndianness;
use serde::ser::{ Serialize, Serializer };
use serde::ser::impls::SeqIteratorVisitor;

#[allow(non_camel_case_types)]
pub enum SubmessageId {
    PAD,
    ACKNACK,
    HEARTBEAT,
    GAP,
    INFO_TS,
    INFO_SRC,
    INFO_REPLY_IP4,
    INFO_DST,
    INFO_REPLY,
    NACK_FRAG,
    HEARTBEAT_FRAG,
    DATA,
    DATA_FRAG,
}

impl Serialize for SubmessageId {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        let val = match *self {
            SubmessageId::PAD => 0x01, /* Pad */
            SubmessageId::ACKNACK => 0x06, /* AckNack */
            SubmessageId::HEARTBEAT => 0x07, /* Heartbeat */
            SubmessageId::GAP => 0x08, /* Gap */
            SubmessageId::INFO_TS => 0x09, /* InfoTimestamp */
            SubmessageId::INFO_SRC => 0x0c, /* InfoSource */
            SubmessageId::INFO_REPLY_IP4 => 0x0d, /* InfoReplyIp4 */
            SubmessageId::INFO_DST => 0x0e, /* InfoDestination */
            SubmessageId::INFO_REPLY => 0x0f, /* InfoReply */
            SubmessageId::NACK_FRAG => 0x12, /* NackFrag */
            SubmessageId::HEARTBEAT_FRAG => 0x13, /* HeartbeatFrag */
            SubmessageId::DATA => 0x15, /* Data */
            SubmessageId::DATA_FRAG => 0x16, /* DataFrag */
        };
        serializer.serialize_u8(val)
    }
}

pub struct Submessage(pub SubmessageId, pub CdrEndianness, pub Vec<u8>);

//bitflags! {
//  pub flags SubmessageType : u8 {
//    const PAD = 0x01, /* Pad */
//    const ACKNACK = 0x06, /* AckNack */
//    const HEARTBEAT = 0x07, /* Heartbeat */
//    const GAP = 0x08, /* Gap */
//    const INFO_TS = 0x09, /* InfoTimestamp */
//    const INFO_SRC = 0x0c, /* InfoSource */
//    const INFO_REPLY_IP4 = 0x0d, /* InfoReplyIp4 */
//    const INFO_DST = 0x0e, /* InfoDestination */
//    const INFO_REPLY = 0x0f, /* InfoReply */
//    const NACK_FRAG = 0x12, /* NackFrag */
//    const HEARTBEAT_FRAG = 0x13, /* HeartbeatFrag */
//    const DATA = 0x15, /* Data */
//    const DATA_FRAG = 0x16, /* DataFrag */
//  }
//}

impl Serialize for Submessage {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        // Write the submessage id
        try!(self.0.serialize(serializer));

        // Write the submessage flags (aka, the endianness)
        let flags : u8 = match self.1 {
            CdrEndianness::Little => 1,
            CdrEndianness::Big => 0
        };
        try!(serializer.serialize_u8(flags));

        // Write all the submessages
        let iter = self.2.iter();
        let visitor = SeqIteratorVisitor::new(iter, Some(self.2.len()));

        serializer.serialize_seq(visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::{CdrSerializer};
    use super::super::super::cdr::ser::CdrEndianness;
    use serde::ser::Serialize;

    #[test]
    fn test_serialize() {
        let submessage = Submessage(
            SubmessageId::DATA,
            CdrEndianness::Little,
            vec![1,2,3,4]
        );
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
}