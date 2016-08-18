use common_types::*;
use super::super::{SubmessageContent,SubmessageId,SubmessageFlags};

pub struct Pad {
    pub size: u16,
}

impl SubmessageContent for Pad {
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