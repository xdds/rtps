pub use super::super::super::common_types::*;

pub type ErrorStr = &'static str;
pub type HistoryCacheResult = Result<(),ErrorStr>;

pub trait HistoryCacheTrait {
    fn new() -> Self;
    fn add_change(&mut self, change: CacheChange) -> HistoryCacheResult;
    fn remove_change(&mut self, change: CacheChange) -> HistoryCacheResult;
    fn get_change();
    fn get_seq_num_min();
    fn get_seq_num_max();
}