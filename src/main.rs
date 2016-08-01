extern crate rstatsd;

use std::thread;
use std::time::Duration;

use rstatsd::servers::udp;
use rstatsd::stats::StatsBuffer;


fn main() {
    println!("totally running statsd! trust me!");
    let queue = StatsBuffer::new();
    let queue_handle = queue.clone();
    thread::spawn(move || {
        udp::collect_udp_messages(("127.0.0.1", 8125), queue_handle);
    });

    loop {
        thread::sleep(Duration::from_millis(10000));
        let buf = queue.flush();
        println!("{:?}", buf)
    }
}
