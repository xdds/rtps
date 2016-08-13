use std::time;
use std::default::Default;

#[derive(Clone,Copy)]
pub struct Duration {
    duration: time::Duration
}

impl Duration {
    pub fn new(secs: u64, nanos: u32) -> Self {
        Duration {
            duration: time::Duration::new(secs, nanos)
        }
    }
}

impl Default for Duration {
    fn default() -> Self {
        Duration{
            duration: time::Duration::new(0,0),
        }
    }
}