#[derive(PartialEq,Debug,Deserialize,Serialize)]
pub struct Timestamp {
    pub seconds: u32,
    pub fraction: u32
}