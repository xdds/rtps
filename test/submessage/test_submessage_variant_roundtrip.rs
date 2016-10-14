use std::io;
use rtps::*;
use serde::{Serialize, Deserialize};
use super::super::factories::Create;

#[test]
fn test_submessage_variant_roundtrip() {
    let variants = [
        "InfoDestination",
        "InfoReply",
        "InfoSource",
        "InfoTimestamp",

        "AckNack",
        "Data",
        "DataFrag",
        "Gap",
        "Heartbeat",
        "HeartbeatFrag",
        "NackFrag",
        "Pad"
    ];

    let mut buf = [0; 64 * 1024];

    for variant_str in variants.iter() {
        let variant = Submessage::create_variant(variant_str);

        let bytes_written = {
            let cursor = io::Cursor::new(&mut buf[..]);
            let mut serializer = cdr::CdrSerializer::new(cursor);
            variant.serialize(&mut serializer).unwrap();
            serializer.write_handle.position() as usize
        };

        let variant_rt: Submessage = {
            let mut cursor = io::Cursor::new(&mut buf[0..bytes_written]);
            let mut de = cdr::CdrDeserializer::new(&mut cursor);
            Deserialize::deserialize(&mut de).unwrap()
        };

        if variant_rt != variant {
            panic!("{:?} failed on {:?}", variant_str, &buf[0..bytes_written]);
        }
        assert_eq!(variant_rt, variant);
    }
}