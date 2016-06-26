extern crate rstatsd;

use std::thread;
use std::time::Duration;

use rstatsd::servers::udp;
use rstatsd::queue::StatsQueue;


fn main() {
    println!("totally running statsd! trust me!");
    let queue = StatsQueue::new();
    let queue_handle = queue.clone();
    let server_handle = thread::spawn(move || {
        udp::collect_udp_messages(("localhost", 34254), queue_handle);
    });

    loop {
        thread::sleep(Duration::from_millis(10000));
        let buf = queue.flush();
        for msg in buf {
            println!("{}", msg);
        }
    }
}
