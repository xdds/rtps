// killer rad re-export
pub use super::super::SubmessageId;
pub use super::super::SubmessageFlags;

pub trait SubmessageContent {
    fn submessage_id() -> SubmessageId;
    fn flags() -> SubmessageFlags;
    fn len() -> u16;
}