use serde::de::Error;

use serde::{ Serialize, Serializer };
use serde;

mod content;
pub use self::content::*;

mod traits;
pub use self::traits::*;

use super::common_types::*;

#[derive(Deserialize,Debug,PartialEq)]
pub struct Submessage {
    pub id: SubmessageId,
    // TODO: single byte is multi-purpose, switch to bitflag
    // add multicast flag
    pub endianness: Endianness,

    pub buf: ArcBuffer,
}

#[derive(Deserialize,Debug,PartialEq)]
pub struct SubmessageV2 {
    pub variant: SubmessageVariant
}

/// 8.3.4.1 Rules Followed by the Message Receiver
#[derive(Debug,PartialEq)]
pub enum SubmessageVariant {
    // Interpreter Submessages
    InfoDestination{ guid_prefix: GuidPrefix },
    InfoReply{ unicast_locator_list: LocatorList /* , multicast_locator_list: Option<LocatorList> TODO: relies on presence of multicast flag above */ },
    InfoSource{ protocol_version: ProtocolVersion, vendor_id: VendorId, guid_prefix: GuidPrefix},
    InfoTimestamp(Timestamp),

    // Entity Submessages
    AckNack { reader_id: EntityId, writer_id: EntityId, reader_sn_state: SequenceNumberSet, count: Count  },
    Data { reader_id: EntityId, writer_id: EntityId, writer_sn: SequenceNumber/*, inline_qos: Option<InlineQOS> */, serialized_payload: ArcBuffer  },
    // TODO: Data frag will require a 'buffer farm' concept so we can process multiple messages then form a single cache_change
    DataFrag { reader_id: EntityId, writer_id: EntityId, writer_sn: SequenceNumber, fragment_start_num: FragmentNumber, fragments_in_submessage: u16, data_size: u32, fragment_size: u16, /*, inline_qos: Option<InlineQOS>, */ serialized_payload: ArcBuffer },
    Gap { reader_id: EntityId, writer_id: EntityId, gap_start: SequenceNumber, gap_list: SequenceNumberSet },
    HeartBeat { reader_id: EntityId, writer_id: EntityId, first_sn: SequenceNumber, last_sn: SequenceNumber, count: Count },
    HeartbeatFrag { reader_id: EntityId, writer_id: EntityId, writer_sn: SequenceNumber, last_fragment_number: FragmentNumber, count: Count  },
    NackFrag
}

impl serde::Deserialize for SubmessageVariant {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
        let kind : SubmessageId = try!(serde::Deserialize::deserialize(deserializer));
        let _ : u8 = try!(serde::Deserialize::deserialize(deserializer));
        // len of message
        let _ : u32 = try!(serde::Deserialize::deserialize(deserializer));

        match kind {
//            SubmessageId::PAD => 0x01, /* Pad */
//            SubmessageId::ACKNACK => 0x06, /* AckNack */
//            SubmessageId::HEARTBEAT => 0x07, /* Heartbeat */
//            SubmessageId::GAP => 0x08, /* Gap */
            SubmessageId::INFO_TS => {
                // Wow, such terse
                Ok(SubmessageVariant::InfoTimestamp(try!(serde::Deserialize::deserialize(deserializer))))
            },
            SubmessageId::INFO_SRC => {
                Ok(SubmessageVariant::InfoSource{
                    protocol_version: try!(serde::Deserialize::deserialize(deserializer)),
                    vendor_id: try!(serde::Deserialize::deserialize(deserializer)),
                    guid_prefix: try!(serde::Deserialize::deserialize(deserializer))
                })
            }, /* InfoSource */
//            SubmessageId::INFO_REPLY_IP4 => 0x0d, /* InfoReplyIp4 */
//            SubmessageId::INFO_DST => 0x0e, /* InfoDestination */
//            SubmessageId::INFO_REPLY => 0x0f, /* InfoReply */
//            SubmessageId::NACK_FRAG => 0x12, /* NackFrag */
//            SubmessageId::HEARTBEAT_FRAG => 0x13, /* HeartbeatFrag */
//            SubmessageId::DATA => 0x15, /* Data */
//            SubmessageId::DATA_FRAG => 0x16, /* DataFrag */
            other => {
                panic!("ahhh: {:?}", other)
            },
        }
    }
}

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

impl serde::Deserialize for SubmessageId {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
        let byte : u8 = try!(serde::Deserialize::deserialize(deserializer));
        match byte {
            0x01 => Ok(SubmessageId::PAD), /* Pad */
            0x06 => Ok(SubmessageId::ACKNACK), /* AckNack */
            0x07 => Ok(SubmessageId::HEARTBEAT), /* Heartbeat */
            0x08 => Ok(SubmessageId::GAP), /* Gap */
            0x09 => Ok(SubmessageId::INFO_TS), /* InfoTimestamp */
            0x0c => Ok(SubmessageId::INFO_SRC), /* InfoSource */
            0x0d => Ok(SubmessageId::INFO_REPLY_IP4), /* InfoReplyIp4 */
            0x0e => Ok(SubmessageId::INFO_DST), /* InfoDestination */
            0x0f => Ok(SubmessageId::INFO_REPLY), /* InfoReply */
            0x12 => Ok(SubmessageId::NACK_FRAG), /* NackFrag */
            0x13 => Ok(SubmessageId::HEARTBEAT_FRAG), /* HeartbeatFrag */
            0x15 => Ok(SubmessageId::DATA), /* Data */
            0x16 => Ok(SubmessageId::DATA_FRAG), /* DataFrag */
            _ => {
                Err(Error::custom(format!("unknown type {:?}", byte)))
//                Err(CdrDeserializerError{ thing: format!("unknown type {:?}", byte) })
            },
        }
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
            Endianness::Little => 1,
            Endianness::Big => 0
        };
        try!(serializer.serialize_u8(flags));

        try!(serializer.serialize_u32(self.buf.len() as u32));
        serializer.serialize_bytes(self.buf.buf())
    }
}