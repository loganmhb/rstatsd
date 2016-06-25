use std::net::UdpSocket;
use std::io;
use std::str;


pub fn listen_on_socket() -> io::Result<()> {
    let mut socket = try!(UdpSocket::bind("localhost:34254"));
    loop {
        let mut buf = [0; 512];
        let (amt, _) = try!(socket.recv_from(&mut buf));
        println!("{}", str::from_utf8(&buf[0..amt]).unwrap());
    }
}
