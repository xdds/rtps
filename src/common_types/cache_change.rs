use super::*;

#[derive(PartialEq)]
pub struct CacheChange {
    kind: ChangeKind,
    writer_guid: Guid,
    instance_handle: InstanceHandle,
    sequence_number: SequenceNumber,
    data: Vec<u8>
}

impl CacheChange {
    pub fn new(kind: ChangeKind, writer_guid: Guid, instance_handle: InstanceHandle,
           sequence_number: SequenceNumber, data: Vec<u8>) -> Self {
        CacheChange {
            kind: kind,
            writer_guid: writer_guid,
            instance_handle: instance_handle,
            sequence_number: sequence_number,
            data: data,
        }
    }
}