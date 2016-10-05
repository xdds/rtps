use common_types::entity_id::*;

pub type GuidPrefix = [u8; 12];

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct Guid {
    pub guid_prefix: GuidPrefix,
    pub entity_id: EntityId
}


impl Guid {
    pub fn new() -> Self {
        Guid {
            guid_prefix: [0; 12],
            entity_id: EntityId {
                entity_key: [0; 3],
                entity_kind: EntityKind::UserUnknown
            }
        }
    }
}