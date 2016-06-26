use std::net::{UdpSocket, ToSocketAddrs};
use std::io;
use std::vec::Vec;
use std::io::Write;
use std::string::String;

use queue::StatsQueue;


pub fn collect_udp_messages<A: ToSocketAddrs>(addr: A, stats_buffer: StatsQueue) {
    let socket = UdpSocket::bind(addr).unwrap();
    loop {
        let mut buf = [0; 512];
        let res = socket.recv_from(&mut buf);
        match res {
            Ok((amt, _)) => {
                let mut v: Vec<u8> = Vec::new();
                v.extend_from_slice(&buf[0..amt]);
                stats_buffer.push(String::from_utf8(v).unwrap())
            },
            Err(e) => write!(io::stderr(), "{}", e).unwrap()
        }
    }
}
