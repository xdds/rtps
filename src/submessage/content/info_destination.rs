use common_types::*;
use super::super::{SubmessageContent,SubmessageId,SubmessageFlags};

pub struct InfoDestination {
    pub guid_prefix: GuidPrefix
}

impl SubmessageContent for InfoDestination {
    fn submessage_id() -> SubmessageId {
        unimplemented!()
    }

    fn flags() -> SubmessageFlags {
        unimplemented!()
    }

    fn len() -> u16 {
        unimplemented!()
    }
}