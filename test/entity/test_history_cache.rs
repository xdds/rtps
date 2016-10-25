use std::{ thread };

use rtps::common_types::*;
use rtps::*;

#[test]
fn test_history_cache() {
    let mut hc = HistoryCache::new();
    let buf1 = ArcBuffer::from_vec(vec![1,2,3]);
    let buf2 = ArcBuffer::from_vec(vec![3,4,5]);

    let cc1 = CacheChange::new(
        ChangeKind::ALIVE,
        Guid::new(),
        InstanceHandle::new(),
        100,
        buf1
    );
    hc.add_change(&cc1).unwrap();

    let cc2 = CacheChange::new(
        ChangeKind::ALIVE,
        Guid::new(),
        InstanceHandle::new(),
        200,
        buf2
    );
    hc.add_change(&cc2).unwrap();

    assert_eq!(hc.get_seq_num_min().unwrap(),100);
    assert_eq!(hc.get_seq_num_max().unwrap(),200);

    hc.remove_change(&cc1).unwrap();

    assert_eq!(hc.get_seq_num_min().unwrap(),200);
    assert_eq!(hc.get_seq_num_max().unwrap(),200);
}

#[test]
fn test_history_cache_wait_for_change() {
    let hc = HistoryCache::new();
    let mut monitor = Monitor::new(hc);

    let buf1 = ArcBuffer::from_vec(vec![1,2,3]);
    let cc1 = CacheChange::new(
        ChangeKind::ALIVE,
        Guid::new(),
        InstanceHandle::new(),
        100,
        buf1
    );

    let buf2 = ArcBuffer::from_vec(vec![3,4,5]);
    let cc2 = CacheChange::new(
        ChangeKind::ALIVE,
        Guid::new(),
        InstanceHandle::new(),
        200,
        buf2
    );

    let threads_monitor = monitor.clone();
    let thread_guard = thread::spawn(move|| {
        threads_monitor.wait().unwrap();

        let cache = threads_monitor.lock().unwrap();
        (cache.get_seq_num_min().unwrap(), cache.get_seq_num_max().unwrap())
    });

    {
        let mut guard = monitor.lock().unwrap();
        guard.add_change(&cc1).unwrap();
    }
    monitor.wakeup_all().unwrap();

    let worked = thread_guard.join().unwrap();
    assert_eq!((100,100), worked);

    let threads_monitor = monitor.clone();
    let thread_guard = thread::spawn(move|| {
        threads_monitor.wait().unwrap();

        let cache = threads_monitor.lock().unwrap();
        (cache.get_seq_num_min().unwrap(), cache.get_seq_num_max().unwrap())
    });

    {
        let mut guard = monitor.lock().unwrap();
        guard.add_change(&cc2).unwrap();
    }
    monitor.wakeup_all().unwrap();

    let worked = thread_guard.join().unwrap();
    assert_eq!((100,200), worked);

}