use common_types::*;
use super::super::{SubmessageContent,SubmessageId,SubmessageFlags};

pub struct DataFrag {
    pub is_key: bool,

    pub reader_id: EntityId,
    pub writer_id: EntityId,

    pub write_sn: SequenceNumber,
    pub inline_qos: Option<ParameterList>,

    pub fragment_starting_num: FragmentNumber,
    pub fragment_in_submessage: u16,
    pub data_size: u32,
    pub fragment_size: u16,

    pub serialized_payload: Vec<u8> // maybe &Serialize or &AppData instead?
}

impl SubmessageContent for DataFrag {
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