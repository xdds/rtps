use std::default::Default;

use super::super::common_types::*;
use super::{ HistoryCacheTrait, HistoryCacheResult };

#[derive(Default)]
pub struct HistoryCache {
    changes: Vec<CacheChange>
}

impl HistoryCacheTrait for HistoryCache {
    fn new() -> Self {
        Default::default()
    }

    fn add_change(&mut self, change: CacheChange) -> HistoryCacheResult {
        self.changes.push(change);
        Ok(())
    }

    fn remove_change(&mut self, change: CacheChange) -> HistoryCacheResult {
        for c in &self.changes {
            if *c == change {

            }
        }

        unimplemented!()
    }

    fn get_change() {

    }

    fn get_seq_num_min() {

    }

    fn get_seq_num_max() {

    }

}