#[allow(non_camel_case_types)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ChangeKind {
    ALIVE,
    NOT_ALIVE_DISPOSED,
    NOT_ALIVE_UNREGISTERED,
}