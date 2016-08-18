use common_types::*;
use super::super::{SubmessageContent,SubmessageId,SubmessageFlags};

pub struct InfoSource {
    pub vendor_id: VendorId,
    pub guid_prefix: GuidPrefix,
}

impl SubmessageContent for InfoSource {
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