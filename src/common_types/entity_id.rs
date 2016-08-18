#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct EntityId {
    pub entity_key: [u8; 3],
    pub entity_kind: u8,
}

