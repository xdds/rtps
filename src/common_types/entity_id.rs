use serde;

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum EntityKind {
    BuiltInUnknown, // 0xc0
    BuiltInParticipant, // 0xc1
    BuiltInWriterWKey, // 0xc2
    BuiltInWriter, // 0xc3
    BuiltInReader, // 0xc4
    BuiltInReaderWKey, // 0xc7

    UserUnknown, // 0x00
    UserWriterWKey, // 0x02
    UserWriter, // 0x03
    UserReader, // 0x04
    UserReaderWKey, // 0x07
}

impl Default for EntityKind {
    fn default() -> Self { EntityKind::UserUnknown }
}

impl serde::Deserialize for EntityKind {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
        let byte : u8 = try!(serde::Deserialize::deserialize(deserializer));
        match byte {
            0xc0 => Ok(EntityKind::BuiltInUnknown),
            0xc1 => Ok(EntityKind::BuiltInParticipant),
            0xc2 => Ok(EntityKind::BuiltInWriterWKey),
            0xc3 => Ok(EntityKind::BuiltInWriter),
            0xc4 => Ok(EntityKind::BuiltInReader),
            0xc7 => Ok(EntityKind::BuiltInReaderWKey),
            0x00 => Ok(EntityKind::UserUnknown),
            0x02 => Ok(EntityKind::UserWriterWKey),
            0x03 => Ok(EntityKind::UserWriter),
            0x04 => Ok(EntityKind::UserReader),
            0x07 => Ok(EntityKind::UserReaderWKey),
            _ => {
                Err(serde::Error::custom(format!("unknown entity kind 0x{:02X}", byte)))
                //                Err(CdrDeserializerError{ thing: format!("unknown type {:?}", byte) })
            },
        }
    }
}

impl serde::Serialize for EntityKind {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
        let val = match *self {
            EntityKind::BuiltInUnknown => 0xc0,
            EntityKind::BuiltInParticipant => 0xc1,
            EntityKind::BuiltInWriterWKey => 0xc2,
            EntityKind::BuiltInWriter => 0xc3,
            EntityKind::BuiltInReader => 0xc4,
            EntityKind::BuiltInReaderWKey => 0xc7,
            EntityKind::UserUnknown => 0x00,
            EntityKind::UserWriterWKey => 0x02,
            EntityKind::UserWriter => 0x03,
            EntityKind::UserReader => 0x04,
            EntityKind::UserReaderWKey => 0x07,
        };
        serializer.serialize_u8(val)
    }
}

#[derive(Default, Debug, PartialEq, Copy, Clone, Deserialize, Serialize)]
pub struct EntityId {
    pub entity_key: [u8; 3],
    pub entity_kind: EntityKind,
}

impl EntityId {
    pub fn builtin_unknown() -> Self {
        EntityId { entity_key: [0,0,0], entity_kind: EntityKind::UserUnknown }
    }
}