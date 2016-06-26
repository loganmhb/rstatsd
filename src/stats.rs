// Basic stat structures
use std::sync::{Mutex, MutexGuard, Arc};
use std::collections::HashMap;

// Representation of a single metric

// TODO: add other metric types
// TODO: add sample rate
enum Metric<'a> {
    Counter { name: &'a str, val: usize }
}

impl<'a> Metric<'a> {
    fn from_string(s: &str) -> Result<Metric, String> {
        let parts: Vec<&str> = s.split(":").collect();
        match parts.as_slice() {
            [name, val] => {
                match val.parse::<usize>() {
                    Ok(i) => Ok(Metric::Counter { name: name, val: i }),
                    Err(_) => Err(format!("Unable to parse value: {}", &val))
                }
            },
            _ => Err(format!("Unexpected format: {} (expected <name>:<val>)", s))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Metrics {
    counters: HashMap<String, usize>
}

impl Metrics {
    fn new() -> Metrics {
        Metrics {
            counters: HashMap::new()
        }
    }

    fn collect(&mut self, item: &str) {
        if item != "" {
            match Metric::from_string(item) {
                Ok(Metric::Counter {name, val}) =>
                    *self.counters.entry(name.to_string()).or_insert(0) += val,
                _ => ()
            }
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
