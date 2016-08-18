use common_types::*;
use super::super::{SubmessageContent,SubmessageId,SubmessageFlags};

pub struct Data {
    pub is_key: bool,

    pub reader_id: EntityId,
    pub writer_id: EntityId,

    pub write_sn: SequenceNumber,
    pub inline_qos: Option<ParameterList>,
    pub serialized_payload: Vec<u8> // maybe &Serialize or &AppData instead?
}

impl SubmessageContent for Data {
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