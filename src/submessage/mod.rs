use cdr::CdrEndianness;
use serde::ser::{ Serialize, Serializer };

mod content;
pub use self::content::*;

mod traits;
pub use self::traits::*;

use super::common_types::*;

pub struct Submessage { pub id: SubmessageId, pub endianness: CdrEndianness, pub buf: ArcBuffer }

#[allow(non_camel_case_types)]
#[derive(Debug,PartialEq)]
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

bitflags! {
    pub flags SubmessageFlags: u8 {
        const LITTLE_ENDIAN = 0x01,
        const ACK_NACK_FINAL_FLAG = 0x02,

        const DATA_INLINE_QOS = 0x02,
        const DATA_DATA = 0x04,
        const DATA_KEY = 0x08,
        const DATA_FRAG_INLINE_QOS = 0x02,
        // eh, will add more as-needed
    }
}

impl Serialize for Submessage {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        // Write the submessage id
        try!(self.id.serialize(serializer));

        // Write the submessage flags (aka, the endianness)
        let flags : u8 = match self.endianness {
            CdrEndianness::Little => 1,
            CdrEndianness::Big => 0
        };
        try!(serializer.serialize_u8(flags));

        try!(serializer.serialize_u32(self.buf.len() as u32));
        serializer.serialize_bytes(self.buf.buf())
    }
}