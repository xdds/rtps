use std::thread;
use std::time;
use std::net::UdpSocket;
use std::sync::{ Arc };
use std::sync::atomic::{ Ordering };
use std::io;

use super::ReaderInitArgs;

use super::super::traits::*;
use super::super::super::common_types::*;

pub struct StatelessReader {
    guid: Guid,
    unicast_locator_list: LocatorList,
    multicast_locator_list: LocatorList,

    handle: Option<SpawnableTaskHandle>,

    socket: Option<Arc<UdpSocket>>
}

impl StatelessReader {
    pub fn new(args: ReaderInitArgs) -> io::Result<Self> {
        let mut reader = StatelessReader {
            guid: args.guid,
            unicast_locator_list: args.unicast_locator_list,
            multicast_locator_list: args.multicast_locator_list,

            handle: None,
            socket: None
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

        let (_, _) = try!(socket.recv_from(buf));
//        let usable_buf = &buf[0..read_size];

//        panic!("buf: {:?}", usable_buf.to_owned());
        Ok(())
    }

    fn stop(&mut self) {
        match self.handle {
            Some(ref handle) => handle.stop_signal.store(true, Ordering::Relaxed),
            None => unreachable!()
        }
    }

    fn join(self) -> thread::Result<()> {
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