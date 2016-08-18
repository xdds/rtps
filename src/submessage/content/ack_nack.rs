use common_types::*;
use super::super::{SubmessageContent,SubmessageId,SubmessageFlags};

pub struct AckNack {
    pub reader_id: EntityId,
    pub writer_id: EntityId,
    pub reader_sns_state: SequenceNumberSet,
    pub count: Count,
}

impl SubmessageContent for AckNack {
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