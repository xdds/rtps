use super::super::super::common_types::Guid;

pub trait EntityTrait {
    fn guid(&self) -> Guid;
}