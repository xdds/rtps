use std::default::Default;

use rtps::*;
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

    let mut common_heartbeat = SubmessageVariant::HeartBeat {
        writer_id: Default::default(),
        reader_id: Default::default(),
        first_sn: 1,
        last_sn: 2,
        count: Default::default(),
    };

    let heartbeat = writer.heartbeat(reader_entity_id);
    if let SubmessageVariant::HeartBeat{ref mut count, ..} = common_heartbeat {
        *count += 1;
    }
    assert_eq!(heartbeat, common_heartbeat);

    let heartbeat2 = writer.heartbeat(reader_entity_id);
    if let SubmessageVariant::HeartBeat{ref mut count, ..} = common_heartbeat {
        *count += 1;
    }
    assert_eq!(heartbeat2, common_heartbeat);

}