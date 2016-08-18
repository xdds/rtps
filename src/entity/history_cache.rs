use std::default::Default;
use std::slice::Iter;

use super::super::common_types::*;
use super::{ HistoryCacheTrait, HistoryCacheResult };

#[derive(Default,Debug)]
pub struct HistoryCache {
    changes: Vec<CacheChange>
}

const NOT_FOUND_ERR: &'static str = "not found";

impl HistoryCacheTrait for HistoryCache {
    fn new() -> Self {
        Default::default()
    }

    fn add_change(&mut self, change: &CacheChange) -> HistoryCacheResult {
        let copy = change.clone();
        self.changes.push(copy);
        Ok(())
    }

    fn remove_change(&mut self, change: &CacheChange) -> HistoryCacheResult {
        let mut index : Option<usize> = None;
        for (i,c) in self.changes.iter().enumerate() {
            if *c == *change {
                index = Some(i);
                break
            }
        }

        match index {
            Some(i) => {
                self.changes.remove(i);
                Ok(())
            },
            None => Err(NOT_FOUND_ERR)
        }
    }

    fn get_seq_num_min(&self) -> Option<SequenceNumber> {
        let min_entry : Option<&CacheChange> = self.changes.iter().min_by_key(|x| x.sequence_number());
        match min_entry {
            Some(c) => Some(c.sequence_number()),
            None => None
        }
    }

    fn get_seq_num_max(&self) -> Option<SequenceNumber> {
        let min_entry : Option<&CacheChange> = self.changes.iter().max_by_key(|x| x.sequence_number());
        match min_entry {
            Some(c) => Some(c.sequence_number()),
            None => None
        }
    }

    fn iter(&self) -> Iter<CacheChange> {
        self.changes.iter()
    }
}
