// 9.4.2.6 SequenceNumberSet

pub type SequenceNumber = u64;

#[derive(PartialEq,Debug,Deserialize)]
pub struct SequenceNumberSet {
    pub base: SequenceNumber,
    pub bitmap: [u8; 32]
}