pub use super::super::SubmessageFlags;

pub trait SubmessageContent {
    fn submessage_id() -> SubmessageId;
    fn flags(&self) -> SubmessageFlags;
    fn len(&self) -> u16;
    fn valid(&self) -> bool;
}