use common_types::*;
use super::super::{SubmessageContent,SubmessageId,SubmessageFlags};

pub struct InfoReply {
    pub unicast_locator_list: LocatorList,
    pub multicast_locator_list: Option<LocatorList>,
}

impl SubmessageContent for InfoReply {
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