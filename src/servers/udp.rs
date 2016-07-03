use std::net::{UdpSocket, ToSocketAddrs};
use std::io;
use std::vec::Vec;
use std::io::Write;
use std::string::String;

use stats::StatsBuffer;


pub fn collect_udp_messages<A: ToSocketAddrs>(addr: A, stats_buffer: StatsBuffer) {
    let socket = UdpSocket::bind(addr).unwrap();
    loop {
        let mut buf = [0; 512];

        // Is it possible that we'll get more than one datagram here?
        let res = socket.recv_from(&mut buf);

        match res {
            Ok((amt, _)) => {
                let mut v: Vec<u8> = Vec::new();
                v.extend_from_slice(&buf[0..amt]);

                for msg in String::from_utf8(v).unwrap().split_terminator("\n") {
                    stats_buffer.collect(msg);
                }
            },
            Err(e) => write!(io::stderr(), "{}", e).unwrap()
        }
    }
}
