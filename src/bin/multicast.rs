extern crate net2;

use std::net::{ UdpSocket, Ipv4Addr };
// use net2::UdpSocketExt;
use std::thread::sleep;
use std::time::Duration;

fn main() {
  let sock = UdpSocket::bind("0.0.0.0:2345").unwrap();
  let local_addr = Ipv4Addr::new(192, 168, 0, 102);

  let multicast_addr = Ipv4Addr::new(239, 255, 0, 1);
  sock.join_multicast_v4(&multicast_addr, &local_addr).unwrap();
  let multicast_addr = Ipv4Addr::new(239, 255, 0, 2);
  sock.join_multicast_v4(&multicast_addr, &local_addr).unwrap();

  let payload = b"sup";
  sock.send_to(&payload[..], (multicast_addr, 5555)).unwrap();

  sleep(Duration::new(10,0));
}