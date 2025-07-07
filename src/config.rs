use std::time::Duration;

use regex::Regex;

use crate::throughput::Throughput;

#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) filter: Option<Regex>,
    pub(crate) warmup: Duration,
    pub(crate) sample_time: Duration,
    pub(crate) sample_count: usize,
    pub(crate) throughput: Option<Throughput>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            warmup: Duration::from_millis(500),
            sample_time: Duration::from_secs(2),
            sample_count: 100,
            throughput: None,
            filter: None,
        }
    }
}
