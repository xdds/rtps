use common_types::*;
use super::super::{SubmessageContent,SubmessageId,SubmessageFlags};

pub struct InfoTimestamp {
    pub timestamp: Timestamp,
}

impl SubmessageContent for InfoTimestamp {
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