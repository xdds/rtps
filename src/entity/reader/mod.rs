use super::super::common_types::*;
use super::EntityTrait;

#[derive(Default)]
pub struct Reader {
    guid: Guid,
}

impl Reader {
    pub fn new() -> Self {
        Reader {
            guid: Guid::new()
        }
    }
}

impl EntityTrait for Reader {
    fn guid(&self) -> Guid {
        self.guid
    }
}