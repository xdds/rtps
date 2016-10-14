#[derive(PartialEq,Debug,Deserialize,Serialize,Clone,Copy)]
pub struct Timestamp {
    pub seconds: u32,
    pub fraction: u32
}