use common_types::*;
use super::super::{SubmessageContent,SubmessageId,SubmessageFlags};

pub struct InfoReply {
    pub unicast_locator_list: LocatorList,
}

impl SubmessageContent for InfoReply {
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