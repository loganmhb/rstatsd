use std::net::{UdpSocket, ToSocketAddrs};
use std::io;
use std::io::Write;
use std::str;


pub fn listen_on_socket<A: ToSocketAddrs>(addr: A) -> {
    let socket = UdpSocket::bind(addr).unwrap();
    loop {
        let mut buf = [0; 512];
        let res = socket.recv_from(&mut buf);
        match res {
            Ok((amt, _)) => println!("{}", str::from_utf8(&buf[0..amt]).unwrap()),
            Err(e) => write!(io::stderr(), "{}", e).unwrap()
        }
    }
}
