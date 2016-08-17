use super::super::super::common_types::*;

pub trait WriterTrait {
    fn new_change(&mut self, change: ChangeKind, handle: InstanceHandle, data: RcBuffer) -> CacheChange;
}