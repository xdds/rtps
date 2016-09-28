use std::default::Default;
use std::{thread, time};
use std::sync::{Arc,Mutex};

use rtps::common_types::*;
use rtps::*;

#[test]
fn test_ping_pong() {
    let mut writer = StatelessWriter::new(WriterInitArgs{
        guid: Guid::new(),
        unicast_locator_list: vec![
            Locator::KIND_UDPv4([0,0,0,0, 0,0,0,0, 0,0,0,0, 127,0,0,1], 8000),
        ],
        .. Default::default()
    });

    writer.new_change(ChangeKind::ALIVE,
                      InstanceHandle::new(),
                      ArcBuffer::from_vec(vec![1,2,3,4])
    );

    let writer_task = SpawnableTaskTrait::spawn(Arc::new(Mutex::new(writer)));
    writer_task.stop();

    let reader = StatelessReader::new(ReaderInitArgs{
        guid: Guid::new(),
        unicast_locator_list: vec![
            Locator::KIND_UDPv4([0,0,0,0, 0,0,0,0, 0,0,0,0, 127,0,0,1], 8000)
        ],
        .. Default::default()
    }).unwrap();
    let reader_task = SpawnableTaskTrait::spawn(Arc::new(Mutex::new(reader)));
    thread::sleep(time::Duration::from_millis(10));
    reader_task.stop();
    writer_task.stop();

    assert!(writer_task.join().unwrap() == ());
    assert!(reader_task.join().unwrap() == ());


}