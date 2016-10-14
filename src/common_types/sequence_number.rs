// 9.4.2.6 SequenceNumberSet

pub type SequenceNumber = u64;

#[derive(PartialEq,Debug,Serialize,Deserialize,Copy,Clone)]
pub struct SequenceNumberSet {
    pub base: SequenceNumber,
    pub bitmap: [u8; 32]
}