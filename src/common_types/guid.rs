#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct Guid {
    guid: [u8; 16]
}


impl Guid {
    pub fn new() -> Self {
        Guid {
            guid: [0; 16]
        }
    }
}