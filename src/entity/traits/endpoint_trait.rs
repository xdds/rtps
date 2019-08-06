use std::{ io, net, time, sync };

use std::default::Default;
use super::super::super::common_types::*;
use super::super::traits::*;

pub trait EndpointTrait : EntityTrait {
    // Getters expected by the spec
    fn topic_kind(&self) -> TopicKind;

    fn reliability_level(&self) -> ReliabilityKind {
        Default::default()
    }

    fn unicast_locator_list(&self) -> &LocatorList;
    fn mut_unicast_locator_list(&mut self) -> &mut LocatorList;

    fn multicast_locator_list(&mut self) -> &mut LocatorList;

    // Added by Xavier
    fn set_socket(&mut self, sync::Arc<net::UdpSocket>);

    fn start_listening(&mut self) -> io::Result<()> {
        let listener: net::UdpSocket = try!(net::UdpSocket::bind("127.0.0.1:0"));

        let locator = if let Ok(net::SocketAddr::V4(addr)) = listener.local_addr() {
            let addr_bytes = addr.ip().octets();
            let mut big_addr = [0; 16];
            big_addr[12] = addr_bytes[0];
            big_addr[13] = addr_bytes[1];
            big_addr[14] = addr_bytes[2];
            big_addr[15] = addr_bytes[3];
            Locator::KIND_UDPv4(u32::from(addr.port()), big_addr)
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "no valid addr".to_owned()));
        };

        // TODO: this here so spawnable task can stop infinite loops on reads (no thread kill mechanism in rust!)
        try!(listener.set_read_timeout(Some(time::Duration::from_millis(5))));

        self.mut_unicast_locator_list().push(locator);
        self.set_socket(sync::Arc::new(listener));

        Ok(())
    }
}