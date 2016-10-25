use std::{ fmt };

use super::super::super::common_types::*;
use super::super::super::sync::Monitor;
use std::slice::Iter;

pub type ErrorStr = &'static str;
pub type HistoryCacheResult = Result<(),ErrorStr>;

pub trait HistoryCacheTrait where Self: Sized {
    fn new() -> Self;
    fn add_change(&mut self, change: &CacheChange) -> HistoryCacheResult;
    fn remove_change(&mut self, change: &CacheChange) -> HistoryCacheResult;
    fn get_seq_num_min(&self) -> Option<SequenceNumber>;
    fn get_seq_num_max(&self) -> Option<SequenceNumber>;
    fn iter(&self) -> Iter<CacheChange>;
}
