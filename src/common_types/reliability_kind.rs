use std::default::Default;

#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub enum ReliabilityKind {
    BEST_EFFORT,
    RELIABLE,
}

impl Default for ReliabilityKind {
    fn default() -> Self {
        ReliabilityKind::BEST_EFFORT
    }
}