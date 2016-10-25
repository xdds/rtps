use std::{ sync, fmt };

use super::super::super::common_types::*;
use std::slice::Iter;

pub type ErrorStr = &'static str;
pub type HistoryCacheResult = Result<(),ErrorStr>;

pub struct Monitor<T>(
    sync::Arc<MonitorInner<T>>
);

struct MonitorInner<T> {
    data: sync::Mutex<T>,
    predicate: (sync::Mutex<bool>, sync::Condvar)
}

impl<T> Monitor<T> {
    pub fn new(monitored: T) -> Self {
        let inner = MonitorInner {
            data: sync::Mutex::new(monitored),
            predicate: (sync::Mutex::new(false), sync::Condvar::new())
        };
        Monitor( sync::Arc::new(inner) )
    }

    pub fn clone(&self) -> Self {
        Monitor(self.0.clone())
    }

    pub fn wait(&self) -> Result<(),sync::PoisonError<sync::MutexGuard<bool>>> {
        let mut open = try!(self.0.predicate.0.lock());
        while !*open {
            open = try!(self.0.predicate.1.wait(open));
        }
        Ok(())
    }

    pub fn lock(&self) -> Result<sync::MutexGuard<T>, sync::PoisonError<sync::MutexGuard<T>>> {
        self.0.data.lock()
    }

    pub fn reset(&mut self) -> Result<(),sync::PoisonError<sync::MutexGuard<bool>>> {
        let mut open = try!(self.0.predicate.0.lock());
        *open = false;
        Ok(())
    }

    pub fn wakeup_all(&mut self) -> Result<(),sync::PoisonError<sync::MutexGuard<bool>>>  {
        let mut open = try!(self.0.predicate.0.lock());
        *open = true;
        self.0.predicate.1.notify_all();
        Ok(())
    }
}

impl<T> fmt::Debug for Monitor<T> {
    fn fmt(&self, mut formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, " meow" )
    }
}


pub trait HistoryCacheTrait where Self: Sized {
    fn new() -> Self;
    fn add_change(&mut self, change: &CacheChange) -> HistoryCacheResult;
    fn remove_change(&mut self, change: &CacheChange) -> HistoryCacheResult;
    fn get_seq_num_min(&self) -> Option<SequenceNumber>;
    fn get_seq_num_max(&self) -> Option<SequenceNumber>;
    fn iter(&self) -> Iter<CacheChange>;
}
