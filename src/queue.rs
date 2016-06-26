use std::sync::{Mutex, MutexGuard, Arc};
use std::vec::Vec;

pub struct StatsQueue {
    buffer: Arc<Mutex<Vec<String>>>
}

impl Clone for StatsQueue {
    fn clone(&self) -> StatsQueue {
        StatsQueue { buffer: self.buffer.clone() }
    }
}

impl StatsQueue {
    pub fn new() -> StatsQueue {
        StatsQueue { buffer: Arc::new(Mutex::new(Vec::new()))}
    }

    pub fn push(&self, item: String) {
        let mut buf: MutexGuard<Vec<String>> = self.buffer.lock().unwrap();
        buf.push(item);
    }

    pub fn flush(&self) -> Vec<String> {
        // Copy everything in the buffer into a Vec, reset the buffer,
        // and return the copy
        let mut handle = self.buffer.lock().unwrap();
        let drained_vals: Vec<String> = handle.clone().into_iter().collect();
        *handle = Vec::new();

        drained_vals
    }
}
