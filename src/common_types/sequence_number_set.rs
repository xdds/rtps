// 9.4.2.6 SequenceNumberSet

use super::SequenceNumber;

#[derive(PartialEq,Debug)]
pub struct SequenceNumberSet {
    pub base: SequenceNumber
}