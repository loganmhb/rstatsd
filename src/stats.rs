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
    fn from_parts(name: &str, val: usize, kind: char) -> Result<Metric, String> {
        match kind {
            'c' => Ok(Metric::Counter {name: name, val: val}),
            _ => Err("Unknown metric type.".to_string())
        }
    }

    fn from_str(s: &str) -> Result<Metric, String> {
        let parts: Vec<&str> = s.split(|c| c == '|' || c == ':').collect();
        match parts.as_slice() {
            &[name, val_str, kind_str] => {
                let val: usize = match val_str.parse() {
                    Ok(i) => i,
                    Err(_) => return Err("Unparsable value.".to_string())
                };
                let kind = try!(kind_str.chars().next().ok_or("Bad format character".to_string()));
                Metric::from_parts(name, val, kind)
            }
            _ => Err("Unrecognized format!".to_string())
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
        match Metric::from_str(item) {
            Ok(Metric::Counter {name, val}) =>
                *self.counters.entry(name.to_string()).or_insert(0) += val,
            Err(e) => println!("{}", e)
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
