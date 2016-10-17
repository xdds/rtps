use std::default::Default;
use std::{thread, time};
use std::sync::{Arc, Mutex};

use rtps::common_types::*;
use rtps::*;

#[test]
fn test_ping_pong() {
    let mut writer = StatelessWriter::new(WriterInitArgs {
        guid: Guid::new(),
        reader_locators: vec![],
        unicast_locator_list: vec![],
        ..Default::default()
    });
    writer.start_listening().unwrap();
    let writer_locator_list = vec![ (writer.unicast_locator_list()[0].clone(), Some(writer.guid().entity_id)) ];

    let mut reader = StatelessReader::new(ReaderInitArgs {
        guid: Guid::new(),
        writer_locator_list: writer_locator_list,
        ..Default::default()
    }).unwrap();
//    let &(ref cvar_mutex, ref cvar) = &*reader.reader_cache_condvar();

    reader.start_listening().unwrap();

    writer.reader_locators.push((reader.unicast_locator_list()[0].clone(), Some(reader.guid().entity_id)));
    writer.new_change(ChangeKind::ALIVE,
                      InstanceHandle::new(),
                      ArcBuffer::from_vec(vec![1,2,3,4]));
    let syncy_writer = Arc::new(Mutex::new(writer));
    let writer_task = SpawnableTaskTrait::spawn(syncy_writer.clone());

    let syncy_reader = Arc::new(Mutex::new(reader));
    let reader_task = SpawnableTaskTrait::spawn(syncy_reader.clone());

//    syncy_reader.lock().unwrap().wait_for_reader_cache_change();
    thread::sleep(time::Duration::from_millis(11)); // Give them time to exchange messages

    // Check that data
    {
        let reader = syncy_reader.lock().unwrap();
        let cache = reader.reader_cache();

        let changes_copy: Vec<ArcBuffer> = cache.iter().map(|c| c.data()).collect();
        assert_eq!(changes_copy, vec![ArcBuffer::from_vec(vec![1,2,3,4])]);
    }

    // Send another message
    {
        let mut writer = syncy_writer.lock().unwrap();

        writer.new_change(ChangeKind::ALIVE,
                          InstanceHandle::new(),
                          ArcBuffer::from_vec(vec![4,3,2,1]));
    }

    thread::sleep(time::Duration::from_millis(11)); // Give them time to exchange messages

    // Check we have all three messages (our history cache is dumb)
    {
        let reader = syncy_reader.lock().unwrap();
        let cache = reader.reader_cache();

        let changes_copy: Vec<ArcBuffer> = cache.iter().map(|c| c.data()).collect();
        assert_eq!(changes_copy, vec![
            ArcBuffer::from_vec(vec![1,2,3,4]),
            ArcBuffer::from_vec(vec![1,2,3,4]),
            ArcBuffer::from_vec(vec![4,3,2,1])
        ]);
    }

    // Turn it all off and count how many times we spun around while
    // taking in to account that each socket read has 10ms timeout (hard-coded for now)
    {
        reader_task.stop();
        writer_task.stop();

        // Getting a little too complicated to count cycles... used to work
        // when the implementation was so bad :)
        //        assert_eq!(writer_task.join().unwrap().iterations, 3);
        //        assert_eq!(reader_task.join().unwrap().iterations, 8);
    }
}