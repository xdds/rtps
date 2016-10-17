use std;

// 9.4.2.6 SequenceNumberSet

pub type SequenceNumber = u64;

#[derive(PartialEq,Debug,Serialize,Deserialize,Copy,Clone,Default)]
pub struct SequenceNumberSet {
    pub base: SequenceNumber,
    pub bitmap: [u8; 32]
}

impl SequenceNumberSet {
    pub fn new() -> Self {
        SequenceNumberSet {
            base: 0,
            bitmap: [0; 32]
        }
    }

    pub fn mark(&mut self, sn: SequenceNumber) -> std::io::Result<()> {
        // TODO: Double check this math
        if self.base - sn > 256 {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "sequence number outside of range".to_owned()));
        }

        let rel = self.base - sn;
        let bin = rel as usize / 8;
        let pos = rel % 8;
        self.bitmap[bin] &= 1 << pos;
        Ok(())
    }
}