use std::thread;
use std::time;
use std::net::UdpSocket;
use std::sync::{ Arc };
use std::sync::atomic::{ Ordering };
use std::io;

use serde;

use super::ReaderInitArgs;

use super::super::HistoryCache;
use super::super::traits::*;
use super::super::super::common_types::*;
use super::super::super::{ Message, SubmessageVariant };

use super::super::super::cdr::{ CdrDeserializer };

pub struct StatelessReader {
    guid: Guid,
    unicast_locator_list: LocatorList,
    multicast_locator_list: LocatorList,

    handle: Option<SpawnableTaskHandle>,

    socket: Option<Arc<UdpSocket>>,

    reader_cache: HistoryCache
}

impl StatelessReader {
    pub fn new(args: ReaderInitArgs) -> io::Result<Self> {
        let mut reader = StatelessReader {
            guid: args.guid,
            unicast_locator_list: args.unicast_locator_list,
            multicast_locator_list: args.multicast_locator_list,

            handle: None,
            socket: None,
            reader_cache: HistoryCache::new(),
        };

        try!(reader.start_listening());
        Ok(reader)
    }

    fn start_listening(&mut self) -> io::Result<()> {
        let listener = try!(UdpSocket::bind("127.0.0.1:9093"));
        try!(listener.set_read_timeout(Some(time::Duration::from_millis(100))));
        self.socket = Some(Arc::new(listener));
        Ok(())
    }
}

impl ReaderTrait for StatelessReader {
}

impl SpawnableTaskTrait for StatelessReader {
    fn werk(&mut self, buf: &mut [u8]) -> io::Result<()> {
        let socket = match self.socket {
            Some(ref sock) => sock,
            None => unreachable!()
        };

        let (size, /* socketAddr */ _) = try!(socket.recv_from(buf));
        let data = &buf[0..size];
        let mut reader = io::Cursor::new(data);

//        panic!("data: {:?}", data);

        let message : Message = match serde::Deserialize::deserialize(&mut CdrDeserializer::new(&mut reader)) {
            Ok(msg) => msg,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "meow"))
        };

        // TODO: should use the two kinds of submessage elements:
        //
        for submessage in message.submessages {
            match submessage.variant {
                SubmessageVariant::Heartbeat{reader_id, ..} => {
//                    panic!("sup from {:?}", reader_id)
                },
                other => {
//                    panic!("mother of god: {:?}", other)
                }
            }
//            panic!("{:?}", submessage);
//            history_cache.add_change(CacheChange::new(ChangeKind::ALIVE, message.));
        }

//        panic!("{:?}", message);

        Ok(())
    }

    fn stop(&mut self) {
        match self.handle {
            Some(ref handle) => handle.stop_signal.store(true, Ordering::Relaxed),
            None => unreachable!()
        }
    }

    fn join(self) -> thread::Result<SpawnableTaskStats> {
        let can_join = self.handle.is_some();
        if can_join {
            self.handle.unwrap().join()
        } else {
            Err(Box::new("cannot join thread. never spawned."))
        }
    }
}

impl EntityTrait for StatelessReader {
    fn guid(&self) -> Guid {
        self.guid
    }
}

impl EndpointTrait for StatelessReader {
    fn topic_kind(&self) -> TopicKind {
        TopicKind::NO_KEY
    }

    fn unicast_locator_list(&self) -> &LocatorList {
        &self.unicast_locator_list
    }

    fn multicast_locator_list(&self) -> &LocatorList {
        &self.multicast_locator_list
    }
}