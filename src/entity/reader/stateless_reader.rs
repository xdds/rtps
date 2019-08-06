use std::thread;
use std::net::UdpSocket;
use std::sync::{ Arc };
use std::sync::atomic::{ Ordering };
use std::io;

use serde;

use super::ReaderInitArgs;

use super::super::HistoryCache;
use super::super::traits::*;
use super::super::super::common_types::*;
use super::super::super::sync::Monitor;
use super::super::super::{ Message, SubmessageVariant };

use super::super::super::cdr::{ CdrDeserializer };

pub struct StatelessReader {
    guid: Guid,
    unicast_locator_list: LocatorList,
    multicast_locator_list: LocatorList,

    handle: Option<SpawnableTaskHandle>,

    // TODO: make private and rely on discovery
    pub writer_locators: Vec<(Locator, Option<EntityId>)>,
    socket: Option<Arc<UdpSocket>>,

    reader_cache: Monitor<HistoryCache>,
}

impl StatelessReader {
    pub fn new(args: ReaderInitArgs) -> io::Result<Self> {
        let reader = StatelessReader {
            guid: args.guid,
            unicast_locator_list: args.unicast_locator_list,
            multicast_locator_list: args.multicast_locator_list,

            handle: None,

            writer_locators: args.writer_locator_list,
            socket: None,

            reader_cache: Monitor::new(HistoryCache::new()),
        };

        Ok(reader)
    }

    pub fn reader_cache(&self) -> Monitor<HistoryCache> {
        self.reader_cache.clone()
    }
}

impl ReaderTrait for StatelessReader {
}

impl SpawnableTaskTrait for StatelessReader {
    fn werk(&mut self, buf: &mut [u8]) -> io::Result<()> {
        for writer_locator in &self.writer_locators {
            if writer_locator.1.is_none() {
                panic!("I don't yet handle resolving their entity id")
            }
        }

        let message : Message = {
            let socket = match self.socket {
                Some(ref sock) => sock,
                None => unreachable!()
            };

            let (size, /* socketAddr */ _) = try!(socket.recv_from(buf));
            let data = &buf[0..size];
            let mut reader = io::Cursor::new(data);

            match serde::Deserialize::deserialize(&mut CdrDeserializer::new(&mut reader)) {
                Ok(msg) => msg,

                // TODO: wouldn't it be neat to just do "Err(err)" becomes "err" and just call "err.into()"
                Err(err) => return Err(err.into())
            }
        };

        // TODO: should use the two kinds of submessage elements:
        //
        let mut had_data = false;
        {
            let mut reader_cache = self.reader_cache.lock().unwrap();

            let mut src_guid_prefix = None;
            for submessage in message.submessages {
                match submessage.variant {
                    SubmessageVariant::Heartbeat { writer_id, first_sn, .. } => {
                        // We can ask the reader's HistoryCache for all values missing:

                        let mut set = SequenceNumberSet::new();
                        set.base = first_sn;

                        for change in reader_cache.iter() {
                            // TODO: construct full guid using guid pulled from previous submessages
                            if change.writer_guid.entity_id != writer_id || change.sequence_number < first_sn {
                                continue
                            }

                            try!(set.mark(change.sequence_number));
                        }
                        // Writer requires a response, let's do it immediately
                    },
                    SubmessageVariant::InfoSource { guid_prefix, .. } => {
                        src_guid_prefix = Some(guid_prefix);
                    },
                    SubmessageVariant::Data { /* reader_id, */ writer_id, /* writer_sn, */ serialized_payload, .. } => {
                        let writer_guid = Guid {
                            guid_prefix: src_guid_prefix.unwrap(),
                            entity_id: writer_id
                        };

                        let change = CacheChange::new(ChangeKind::ALIVE, writer_guid, InstanceHandle::new(), 0, serialized_payload);

                        reader_cache.add_change(&change).unwrap();
                        had_data = true;
                    },
                    other => {
                        panic!("mother of god: {:?}", other)
                    }
                }
            }
        }

        if had_data {
            self.reader_cache.wakeup_all().unwrap();
        }

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

    fn set_socket(&mut self, sock: Arc<UdpSocket>) {
        self.socket = Some(sock);
    }

    fn unicast_locator_list(&self) -> &LocatorList {
        &self.unicast_locator_list
    }

    fn mut_unicast_locator_list(&mut self) -> &mut LocatorList {
        &mut self.unicast_locator_list
    }

    fn multicast_locator_list(&mut self) -> &mut LocatorList {
        &mut self.multicast_locator_list
    }
}