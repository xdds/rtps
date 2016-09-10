use common_types::*;
use super::super::{SubmessageContent,SubmessageId,SubmessageFlags};

#[derive(Debug,PartialEq)]
pub struct Heartbeat {
    pub is_key: bool,

    pub reader_id: EntityId,
    pub writer_id: EntityId,

    pub first_sn: SequenceNumber,
    pub last_sn: SequenceNumber,

    pub count: Count
}

impl SubmessageContent for Heartbeat {
    fn submessage_id() -> SubmessageId {
        unimplemented!()
    }

    fn flags(&self) -> SubmessageFlags {
        unimplemented!()
    }

    fn len(&self) -> u16 {
        unimplemented!()
    }

    fn valid(&self) -> bool {
        unimplemented!()
    }
}