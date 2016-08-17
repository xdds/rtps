use super::super::super::common_types::*;

pub type ErrorStr = &'static str;
pub type HistoryCacheResult = Result<(),ErrorStr>;

pub trait HistoryCacheTrait {
    fn new() -> Self;
    fn add_change(&mut self, change: &CacheChange) -> HistoryCacheResult;
    fn remove_change(&mut self, change: &CacheChange) -> HistoryCacheResult;
    fn get_seq_num_min(&self) -> Option<SequenceNumber>;
    fn get_seq_num_max(&self) -> Option<SequenceNumber>;
}