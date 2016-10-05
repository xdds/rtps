pub type ParameterList = Vec<Parameter>;

#[derive(PartialEq,Debug)]
pub struct Parameter {
    pub parameter_id: u32,
    pub length: u16,
    pub value: Vec<u8>,
}