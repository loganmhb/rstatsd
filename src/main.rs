extern crate rstatsd;

use rstatsd::servers::udp;

fn main() {
    println!("totally running statsd! trust me!");
    udp::listen_on_socket();
}
