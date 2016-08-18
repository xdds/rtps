use std::default::Default;

use rtps::{ StatelessWriter, WriterInitArgs, WriterTrait };
use rtps::common_types::*;

#[test]
fn test_stateless_writer() {
    let mut writer =  StatelessWriter::new(WriterInitArgs {
        unicast_locator_list: vec![
            Locator::BUF(vec![])
        ],
        guid: Guid::new(),
        .. Default::default()
    });

    writer.new_change(
        ChangeKind::ALIVE, InstanceHandle::new(),
        RcBuffer::from_vec(vec![])
    );

    let reader_entity_id : EntityId = Default::default();

    let heartbeat = writer.heartbeat(reader_entity_id);
    panic!(format!("{:?}", heartbeat));
}