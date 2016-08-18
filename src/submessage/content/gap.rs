use common_types::*;
use super::super::{SubmessageContent,SubmessageId,SubmessageFlags};

pub struct Gap {
    pub is_key: bool,

    pub reader_id: EntityId,
    pub writer_id: EntityId,

    pub gap_start: SequenceNumber,
    pub gap_stop: SequenceNumber,
}

impl SubmessageContent for Gap {
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