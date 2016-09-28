use std::default::Default;

use rtps::{ StatelessWriter, WriterTrait, Heartbeat };
use rtps::common_types::*;

use factories::Create;

#[test]
fn test_stateless_writer_heartbeat_increments_count() {
    let mut writer : StatelessWriter = Create::create();

    writer.new_change(
        ChangeKind::ALIVE, InstanceHandle::new(),
        ArcBuffer::from_vec(vec![])
    );
    writer.new_change(
        ChangeKind::ALIVE, InstanceHandle::new(),
        ArcBuffer::from_vec(vec![])
    );

    let reader_entity_id : EntityId = Default::default();

    let common_heartbeat = Heartbeat {
        writer_id: Default::default(),
        reader_id: Default::default(),
        is_key: false,
        first_sn: 1,
        last_sn: 2,
        count: Default::default(),
    };

    let heartbeat = writer.heartbeat(reader_entity_id);
    assert_eq!(heartbeat, Heartbeat {
        count: 0,
        .. common_heartbeat
    });

    let heartbeat2 = writer.heartbeat(reader_entity_id);
    assert_eq!(heartbeat2, Heartbeat {
        count: 1,
        .. common_heartbeat
    });

}