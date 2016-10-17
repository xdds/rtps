use super::*;
use super::super::{ Submessage, SubmessageVariant };

#[derive(Clone,Debug,)]
pub struct CacheChange {
    kind: ChangeKind,
    pub writer_guid: Guid,
    instance_handle: InstanceHandle,
    pub sequence_number: SequenceNumber,
    data: ArcBuffer
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
           sequence_number: SequenceNumber, data: ArcBuffer) -> Self {
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

    pub fn to_submessage(&self, reader_id: Guid) -> Submessage {
        Submessage {
            variant: SubmessageVariant::Data {
                reader_id: reader_id.entity_id,
                writer_id: self.writer_guid.entity_id,
                writer_sn: self.sequence_number,
                serialized_payload: self.data.clone(),

            }
        }
    }

    pub fn data(&self) -> ArcBuffer {
        self.data.clone()
    }
}