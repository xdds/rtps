use serde::{ Serialize, Serializer };
use serde;

use super::common_types::*;

#[derive(Deserialize,Debug,PartialEq)]
pub struct Submessage {
    pub variant: SubmessageVariant
}

/// 8.3.4.1 Rules Followed by the Message Receiver
#[derive(Debug,PartialEq)]
pub enum SubmessageVariant {
    // Interpreter Submessages
    InfoDestination( GuidPrefix ),
    // TODO: not positive the on-wire format of a Locator_t*. Not length-prefixed?
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
    HeartbeatFrag { reader_id: EntityId, writer_id: EntityId, writer_sn: SequenceNumber, last_fragment_number: FragmentNumber, count: Count },
    NackFrag { reader_id: EntityId, writer_id: EntityId, writer_sn: SequenceNumber, fragment_number_state: FragmentNumberSet, count: Count },
    Pad
}

impl serde::Deserialize for SubmessageVariant {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
        let kind : SubmessageId = try!(serde::Deserialize::deserialize(deserializer));
        let _ : u8 = try!(serde::Deserialize::deserialize(deserializer));
        // len of message. TODO: use to confirm the proper number of bytes are read
        let _ : u32 = try!(serde::Deserialize::deserialize(deserializer));

        match kind {
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
            },
            SubmessageId::INFO_REPLY => {
                Ok(SubmessageVariant::InfoReply{
                    unicast_locator_list: try!(serde::Deserialize::deserialize(deserializer))
                })
            },
            SubmessageId::INFO_DST => {
                Ok(SubmessageVariant::InfoDestination(try!(serde::Deserialize::deserialize(deserializer))))
            },
            SubmessageId::INFO_REPLY_IP4 => {
                Err(serde::Error::custom("we don't do ipv4 specialization yet"))
//                0x0f
            },

            SubmessageId::ACKNACK => {
                Ok(SubmessageVariant::AckNack {
                    reader_id: try!(serde::Deserialize::deserialize(deserializer)),
                    writer_id: try!(serde::Deserialize::deserialize(deserializer)),
                    reader_sn_state: try!(serde::Deserialize::deserialize(deserializer)),
                    count: try!(serde::Deserialize::deserialize(deserializer)),
                })
            },
            SubmessageId::DATA => {
                Ok(SubmessageVariant::Data {
                    reader_id: try!(serde::Deserialize::deserialize(deserializer)),
                    writer_id: try!(serde::Deserialize::deserialize(deserializer)),
                    writer_sn: try!(serde::Deserialize::deserialize(deserializer)),
                    serialized_payload: try!(serde::Deserialize::deserialize(deserializer)),
                })
            },
            SubmessageId::DATA_FRAG => {
                Ok(SubmessageVariant::DataFrag {
                    reader_id: try!(serde::Deserialize::deserialize(deserializer)),
                    writer_id: try!(serde::Deserialize::deserialize(deserializer)),
                    writer_sn: try!(serde::Deserialize::deserialize(deserializer)),

                    fragment_start_num: try!(serde::Deserialize::deserialize(deserializer)),
                    fragments_in_submessage: try!(serde::Deserialize::deserialize(deserializer)),
                    data_size: try!(serde::Deserialize::deserialize(deserializer)),
                    fragment_size: try!(serde::Deserialize::deserialize(deserializer)),

                    serialized_payload: try!(serde::Deserialize::deserialize(deserializer)),
                })
            },
            SubmessageId::HEARTBEAT => {
                Ok(SubmessageVariant::HeartBeat{
                    reader_id: try!(serde::Deserialize::deserialize(deserializer)),
                    writer_id: try!(serde::Deserialize::deserialize(deserializer)),
                    first_sn: try!(serde::Deserialize::deserialize(deserializer)),
                    last_sn: try!(serde::Deserialize::deserialize(deserializer)),
                    count: try!(serde::Deserialize::deserialize(deserializer)),
                })
            },
            SubmessageId::HEARTBEAT_FRAG => {
                Ok(SubmessageVariant::HeartbeatFrag {
                    reader_id: try!(serde::Deserialize::deserialize(deserializer)),
                    writer_id: try!(serde::Deserialize::deserialize(deserializer)),
                    writer_sn: try!(serde::Deserialize::deserialize(deserializer)),
                    last_fragment_number: try!(serde::Deserialize::deserialize(deserializer)),
                    count: try!(serde::Deserialize::deserialize(deserializer)),
                })
            },
            SubmessageId::GAP => {
                Ok(SubmessageVariant::Gap{
                    reader_id: try!(serde::Deserialize::deserialize(deserializer)),
                    writer_id: try!(serde::Deserialize::deserialize(deserializer)),
                    gap_start: try!(serde::Deserialize::deserialize(deserializer)),
                    gap_list: try!(serde::Deserialize::deserialize(deserializer)),
                })
            },

            SubmessageId::NACK_FRAG => {
                Ok(SubmessageVariant::NackFrag{
                    reader_id: try!(serde::Deserialize::deserialize(deserializer)),
                    writer_id: try!(serde::Deserialize::deserialize(deserializer)),
                    writer_sn: try!(serde::Deserialize::deserialize(deserializer)),
                    fragment_number_state: try!(serde::Deserialize::deserialize(deserializer)),
                    count: try!(serde::Deserialize::deserialize(deserializer)),
                })
            },
            SubmessageId::PAD => {
                Ok(SubmessageVariant::Pad)
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug,PartialEq)]
enum SubmessageId {
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
            _ => Err(serde::Error::custom(format!("unknown type {:?}", byte))),
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
        match self.variant {
            SubmessageVariant::InfoTimestamp(ref ts) => {
                try!(serializer.serialize_u8(0x09));
                try!(ts.serialize(serializer));
                Ok(())
            },
            SubmessageVariant::InfoSource{ protocol_version, vendor_id, guid_prefix } => {
                try!(serializer.serialize_u8(0x0c));
                try!(protocol_version.serialize(serializer));
                try!(vendor_id.serialize(serializer));
                try!(guid_prefix.serialize(serializer));
                Ok(())
            },
//            SubmessageId::INFO_REPLY => {
//                Ok(SubmessageVariant::InfoReply{
//                    unicast_locator_list: try!(serde::Deserialize::deserialize(deserializer))
//                })
//            },
//            SubmessageId::INFO_DST => {
//                Ok(SubmessageVariant::InfoDestination(try!(serde::Deserialize::deserialize(deserializer))))
//            },
//            SubmessageId::INFO_REPLY_IP4 => {
//                Err(serde::Error::custom("we don't do ipv4 specialization yet"))
//                //                0x0f
//            },
//
//            SubmessageId::ACKNACK => {
//                Ok(SubmessageVariant::AckNack {
//                    reader_id: try!(serde::Deserialize::deserialize(deserializer)),
//                    writer_id: try!(serde::Deserialize::deserialize(deserializer)),
//                    reader_sn_state: try!(serde::Deserialize::deserialize(deserializer)),
//                    count: try!(serde::Deserialize::deserialize(deserializer)),
//                })
//            },
            SubmessageVariant::Data{ reader_id, writer_id, writer_sn /*, inline_qos: Option<InlineQOS> */, ref serialized_payload } => {
                try!(serializer.serialize_u8(0x15));
                try!(reader_id.serialize(serializer));
                try!(writer_id.serialize(serializer));
                try!(writer_sn.serialize(serializer));
                try!(serialized_payload.serialize(serializer));
                Ok(())
            },
//            SubmessageId::DATA_FRAG => {
//                Ok(SubmessageVariant::DataFrag {
//                    reader_id: try!(serde::Deserialize::deserialize(deserializer)),
//                    writer_id: try!(serde::Deserialize::deserialize(deserializer)),
//                    writer_sn: try!(serde::Deserialize::deserialize(deserializer)),
//
//                    fragment_start_num: try!(serde::Deserialize::deserialize(deserializer)),
//                    fragments_in_submessage: try!(serde::Deserialize::deserialize(deserializer)),
//                    data_size: try!(serde::Deserialize::deserialize(deserializer)),
//                    fragment_size: try!(serde::Deserialize::deserialize(deserializer)),
//
//                    serialized_payload: try!(serde::Deserialize::deserialize(deserializer)),
//                })
//            },
//            SubmessageId::HEARTBEAT => {
//                Ok(SubmessageVariant::HeartBeat{
//                    reader_id: try!(serde::Deserialize::deserialize(deserializer)),
//                    writer_id: try!(serde::Deserialize::deserialize(deserializer)),
//                    first_sn: try!(serde::Deserialize::deserialize(deserializer)),
//                    last_sn: try!(serde::Deserialize::deserialize(deserializer)),
//                    count: try!(serde::Deserialize::deserialize(deserializer)),
//                })
//            },
//            SubmessageId::HEARTBEAT_FRAG => {
//                Ok(SubmessageVariant::HeartbeatFrag {
//                    reader_id: try!(serde::Deserialize::deserialize(deserializer)),
//                    writer_id: try!(serde::Deserialize::deserialize(deserializer)),
//                    writer_sn: try!(serde::Deserialize::deserialize(deserializer)),
//                    last_fragment_number: try!(serde::Deserialize::deserialize(deserializer)),
//                    count: try!(serde::Deserialize::deserialize(deserializer)),
//                })
//            },
//            SubmessageId::GAP => {
//                Ok(SubmessageVariant::Gap{
//                    reader_id: try!(serde::Deserialize::deserialize(deserializer)),
//                    writer_id: try!(serde::Deserialize::deserialize(deserializer)),
//                    gap_start: try!(serde::Deserialize::deserialize(deserializer)),
//                    gap_list: try!(serde::Deserialize::deserialize(deserializer)),
//                })
//            },
//
//            SubmessageId::NACK_FRAG => {
//                Ok(SubmessageVariant::NackFrag{
//                    reader_id: try!(serde::Deserialize::deserialize(deserializer)),
//                    writer_id: try!(serde::Deserialize::deserialize(deserializer)),
//                    writer_sn: try!(serde::Deserialize::deserialize(deserializer)),
//                    fragment_number_state: try!(serde::Deserialize::deserialize(deserializer)),
//                    count: try!(serde::Deserialize::deserialize(deserializer)),
//                })
//            },
//            SubmessageId::PAD => {
//                Ok(SubmessageVariant::Pad)
//            }
            _ => panic!(format!("unsupported variant {:?}", self.variant))

        }
    }
}