use super::super::super::common_types::*;

pub trait EntityTrait {
    fn guid(&self) -> Guid;
}