use rtps::common_types::*;
use rtps::*;

#[test]
fn test_history_cache() {
    let mut hc = HistoryCache::new();
    let cc1 = CacheChange::new(
        ChangeKind::ALIVE,
        Guid::new(),
        InstanceHandle::new(),
        100,
        &vec![1,2,3]
    );
    hc.add_change(&cc1).unwrap();

    let cc2 = CacheChange::new(
        ChangeKind::ALIVE,
        Guid::new(),
        InstanceHandle::new(),
        200,
        &vec![4,5,6]
    );
    hc.add_change(&cc2).unwrap();

    assert_eq!(hc.get_seq_num_min().unwrap(),100);
    assert_eq!(hc.get_seq_num_max().unwrap(),200);

    hc.remove_change(&cc1).unwrap();

    assert_eq!(hc.get_seq_num_min().unwrap(),200);
    assert_eq!(hc.get_seq_num_max().unwrap(),200);
}