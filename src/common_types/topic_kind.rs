#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub enum TopicKind {
    NO_KEY,
    WITH_KEY
}

impl Default for TopicKind {
    fn default() -> Self {
        TopicKind::NO_KEY
    }
}