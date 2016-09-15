use std::default::Default;

use rtps::common_types::*;
use rtps::*;

#[test]
fn test_ping_pong() {
    let writer = StatelessWriter::new(WriterInitArgs{
        guid: Guid::new(),
        unicast_locator_list: vec![
            Locator::KIND_UDPv4([0,0,0,0, 0,0,0,0, 0,0,0,0, 127,0,0,1], 8000)
        ],
        .. Default::default()
    });

    let reader = StatelessReader::new(ReaderInitArgs{
        guid: Guid::new(),
        unicast_locator_list: vec![
            Locator::KIND_UDPv4([0,0,0,0, 0,0,0,0, 0,0,0,0, 127,0,0,1], 8000)
        ],
        .. Default::default()
    });


}