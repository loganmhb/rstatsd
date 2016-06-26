// Basic stat structures
use std::sync::{Mutex, MutexGuard, Arc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Metrics {
    counters: HashMap<String, i64>
}

impl Metrics {
    fn new() -> Metrics {
        Metrics {
            counters: HashMap::new()
        }
    }

    fn collect(&mut self, item: &str) {
        if item != "" {
            *self.counters.entry(item.to_string()).or_insert(0) += 1;
        }
    }
}

/// StatsBuffer is a threadsafe wrapper around Metrics.
pub struct StatsBuffer {
    metrics: Arc<Mutex<Metrics>>
}

impl Clone for StatsBuffer {
    fn clone(&self) -> StatsBuffer {
        StatsBuffer { metrics: self.metrics.clone() }
    }
}

impl StatsBuffer {
    pub fn new() -> StatsBuffer {
        let m = Metrics::new();
        StatsBuffer { metrics: Arc::new(Mutex::new(m)) }
    }

    pub fn collect(&self, item: &str) {
        let mut metrics: MutexGuard<Metrics> = self.metrics.lock().unwrap();
        (*metrics).collect(item);
    }

    pub fn flush(&self) -> Metrics {
        let mut handle = self.metrics.lock().unwrap();
        let drained_vals: Metrics = handle.clone();
        *handle = Metrics::new();

        drained_vals
    }
}
