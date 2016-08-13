use std::default::Default;

#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub enum ProtocolVersion {
    PROTOCOL_VERSION_2_2
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        ProtocolVersion::PROTOCOL_VERSION_2_2
    }
}