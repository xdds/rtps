use rtps::message::types::*;
use rtps::message::header::*;

#[test]
fn test_serialize_old() {
    let h = Header {
        protocol_version: ProtocolVersion(2, 2),
        vendor_id: VendorId(2),
        guid_prefix: GUIDPrefix(GUIDPREFIX_UNKNOWN)
    };
    let mut buf = vec![];
    h.serialize_old(&mut buf).unwrap();

    let expected = vec![
      // 'RTPS'
      82, 84, 80, 83,
      // protocol_version
      2, 2,
      // vendor_id
      2, 0,
      // guid_prefix
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];

    assert_eq!(buf, expected);
}