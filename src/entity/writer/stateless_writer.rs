/// RTPS Writer
/// Entity diagram found in formal-14-09-01 on manual page 71
/// Behavior statemachine found in formal-14-09-01

use serde::Serialize;

use std::error::Error;
use std::thread;
use std::sync::atomic::{Ordering};
use std::io;
use std::io::{Cursor};

use std::net::UdpSocket;
use std;

use super::super::super::common_types::*;
use super::super::*;
use super::super::super::submessage::*;
use super::super::super::message::*;
use super::super::super::cdr::*;

pub struct StatelessWriter {
    guid: Guid,
    /// List of unicast locators (transport, address, port combinations) that can be used to send messages to this Endpoint. The list may be empty.
    unicast_locator_list: LocatorList,
    /// List of multicast locators (transport, address, port combinations) that can be used to send messages to this Endpoint. The list may be empty.
    multicast_locator_list: LocatorList,
    reliability_level: ReliabilityKind,
    topic_kind: TopicKind,
    push_mode: bool,
    heartbeat_period: Duration,
    nack_response_delay: Duration,
    nack_suppression_duration: Duration,
    writer_cache: HistoryCache,

    last_change_sequence_number: SequenceNumber,
    heartbeat_count: u32,

    //    resend_data_period: Duration,

    // Better name would be `reader_locator_list` to match other
    // properties. But this follows the RTPS spec naming.
    // Also a slightly different type, includes EntityId
    // if that's been negotiated
    // TODO: make private and rely on discovery
    pub reader_locators: Vec<(Locator, Option<EntityId>)>,

    socket: Option<std::sync::Arc<UdpSocket>>,

    handle: Option<SpawnableTaskHandle>
}

impl StatelessWriter {
    /// You must provide at least one entry in the unicast locator list
    /// or it will fail to produce heartbeats.
    /// TODO: change `new() -> Self` to `new() -> Option<Self>`
    pub fn new(init_args: WriterInitArgs) -> Self {
        StatelessWriter {
            guid: init_args.guid,
            unicast_locator_list: init_args.unicast_locator_list,
            multicast_locator_list: init_args.multicast_locator_list,
            reliability_level: init_args.reliability_level,
            topic_kind: init_args.topic_kind,
            push_mode: init_args.push_mode,
            heartbeat_period: init_args.heartbeat_period,
            nack_response_delay: init_args.nack_response_delay,
            nack_suppression_duration: init_args.nack_suppression_duration,

            writer_cache: HistoryCache::new(),

            last_change_sequence_number: 0,
            heartbeat_count: 0,

            reader_locators: init_args.reader_locators,

            socket: None,
            handle: None
        }
    }

    fn push_mode(&self) -> bool {
        self.push_mode
    }

    fn heartbeat_period(&self) -> Duration {
        self.heartbeat_period
    }

    fn nack_response_delay(&self) -> Duration {
        self.nack_response_delay
    }

    fn nack_suppression_duration(&self) -> Duration {
        self.nack_response_delay
    }

    pub fn heartbeat(&mut self, reader_id: EntityId) -> SubmessageVariant {
        let max = self.writer_cache.get_seq_num_max().unwrap_or(0);
        let min = self.writer_cache.get_seq_num_min().unwrap_or(0);

        let heartbeat = SubmessageVariant::Heartbeat {
            reader_id,
            writer_id: self.guid.entity_id,

            first_sn: min,
            last_sn: max,

            count: self.heartbeat_count
        };

        self.heartbeat_count += 1;

        heartbeat
    }

    pub fn serialize_message<'a>(buf: &'a mut [u8], message: &Message) -> Result<&'a [u8], io::Error> {
        let position = {
            let new_slice = &mut buf[..];
            let writeable_buf = Cursor::new(new_slice);

            let mut serializer = CdrSerializer {
                endianness: Endianness::Big,
                write_handle: writeable_buf
            };

            match message.serialize(&mut serializer) {
                Ok(_) => (),
                Err(err) => {
                    return Err(io::Error::new(io::ErrorKind::Other, err.description()))
                },
            }

            serializer.write_handle.position() as usize
        };

        Ok(&buf[0..position])
    }
}

impl EntityTrait for StatelessWriter {
    fn guid(&self) -> Guid {
        self.guid
    }
}

impl EndpointTrait for StatelessWriter {
    fn topic_kind(&self) -> TopicKind {
        self.topic_kind
    }

    fn set_socket(&mut self, sock: std::sync::Arc<UdpSocket>) {
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

impl WriterTrait for StatelessWriter {
    fn new_change(&mut self, kind: ChangeKind, handle: InstanceHandle, data: ArcBuffer) -> CacheChange {
        self.last_change_sequence_number += 1;

        let change = CacheChange::new(kind, self.guid, handle, self.last_change_sequence_number, data);
        self.writer_cache.add_change(&change).unwrap();
        change
    }
}

impl SpawnableTaskTrait for StatelessWriter {
    fn werk(&mut self, buf: &mut [u8]) -> io::Result<()> {
        // 1. Spam the reader with your id, locator, and a heartbeat requesting their cache state back
        // 2. Read their

        // TODO: `self.reader_locators.clone()` necessary because `self.heartbeat` wants mut ref.
        for (reader_locator, reader) in self.reader_locators.clone() {
            // TODO: Skip the intermediate vec and just serialize to the buf
            let mut submsgs = vec![];

            // Who I am
            submsgs.push(SubmessageVariant::InfoSource {
                protocol_version: ProtocolVersion::VERSION_2_2,
                vendor_id: [20,10],
                guid_prefix: self.guid.guid_prefix
            });

            for change in self.writer_cache.iter() {
                submsgs.push(change.to_submessage(self.guid).variant)
            }

            let heartbeat = match reader {
                Some(reader) => self.heartbeat(reader),
                None => self.heartbeat(EntityId::builtin_unknown())
            };
            submsgs.push(heartbeat);

            let used_buf = try!(StatelessWriter::serialize_message(buf, &Message::new(submsgs.into_iter().map(|smv| Submessage{variant: smv}).collect())));
            try!(reader_locator.write(used_buf));

            /* TODO: move to a transaction where the count only goes up once. for now we increment on each host
            at the writer level.Perhaps we could do count at the ReaderLocator level?
            let heartbeat = match reader {
                Some(reader) => self.heartbeat(reader),
                None => self.heartbeat(EntityId::builtin_unknown())
            };

            for location in &self.unicast_locator_list {
                let used_buf = try!(StatelessWriter::serialize_message(buf, &Message::new(vec![Submessage{ variant: heartbeat.clone() }])));
                try!(location.write(used_buf));
            } */
        }

        std::thread::sleep(std::time::Duration::from_millis(10));

        let socket = match self.socket {
            Some(ref sock) => sock,
            None => unreachable!()
        };

        let (size, /* socketAddr */ _) = try!(socket.recv_from(buf));
        let data = &buf[0..size];
        let mut _reader = io::Cursor::new(data);

        /*
        for change in self.writer_cache.iter() {
            let position = {
                let new_slice = &mut buf[..];
                let writeable_buf = Cursor::new(new_slice);

                let mut serializer = CdrSerializer {
                    endianness: Endianness::Big,
                    write_handle: writeable_buf
                };

                let message = Message::new(vec![
                    change.to_submessage(self.guid)
                ]);
                // need control flow to create my non-io::Error stuff.
                match message.serialize(&mut serializer) {
                    Ok(_) => (),
                    Err(err) => {
                        return Err(io::Error::new(io::ErrorKind::Other, err.description()))
                    },
                }

                serializer.write_handle.position() as usize
            };

            let used_buf: &[u8] = &buf[0..position];

            for location in &mut self.unicast_locator_list {
                try!(location.write(used_buf));
            }

        }
        */

        Ok(())
    }

    fn stop(&mut self) {
        match self.handle {
            Some(ref mut handle) => {
                handle.stop_signal.store(true, Ordering::Relaxed);
            },
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