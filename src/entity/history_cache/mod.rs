use super::super::common_types::*;

pub type Change = ();

pub struct HistoryCache {
    changes: Vec<Change>
}

impl HistoryCache {
    pub fn new() -> Self {
        HistoryCache {
            changes: vec![]
        }
    }

    pub fn add(&self, _: ChangeKind, _: InstanceHandle, _: Vec<u8>) {
        unimplemented!()
    }
}