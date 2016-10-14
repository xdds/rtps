pub type FragmentNumber = u32;

#[derive(PartialEq,Debug,Deserialize,Serialize,Copy,Clone)]
pub struct FragmentNumberSet {
    pub base: FragmentNumber,
    pub set: [u8; 32], // 256 bits of data. TODO: could be a bitset?
}