use super::*;

#[derive(Clone,Debug)]
pub struct CacheChange {
    kind: ChangeKind,
    writer_guid: Guid,
    instance_handle: InstanceHandle,
    sequence_number: SequenceNumber,
    data: RcBuffer
}

impl PartialEq for CacheChange {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind &&
            self.writer_guid == other.writer_guid &&
            self.instance_handle == other.instance_handle &&
            self.sequence_number == other.sequence_number
    }
}

impl CacheChange {
    pub fn new(kind: ChangeKind, writer_guid: Guid, instance_handle: InstanceHandle,
           sequence_number: SequenceNumber, data: RcBuffer) -> Self {
        CacheChange {
            kind: kind,
            writer_guid: writer_guid,
            instance_handle: instance_handle,
            sequence_number: sequence_number,
            data: data.clone(),
        }
    }

    pub fn sequence_number(&self) -> SequenceNumber {
        self.sequence_number
    }
}