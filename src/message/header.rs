use common_types::*;

const RTPS: &'static [u8; 4] = b"RTPS";
pub const GUIDPREFIX_UNKNOWN: [u8; 12] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

pub struct Header {
    pub protocol_version: ProtocolVersion,
    pub vendor_id: VendorId,
    pub guid_prefix: GuidPrefix
}
