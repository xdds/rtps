use super::FragmentNumber;

pub struct FragmentNumberSet {
    pub base: FragmentNumber,
    pub set: [u8; 32], // 256 bits of data. TODO: could be a bitset?
}