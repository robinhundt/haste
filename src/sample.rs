use std::time::Duration;

pub(crate) struct Sample {
    pub(crate) iter_time_ns: f64,
    #[allow(dead_code)]
    pub(crate) sample_size: usize,
}

impl Sample {
    pub(crate) fn from_duration(duration: Duration, sample_size: usize) -> Self {
        let iter_time_ns = (duration.as_nanos() as f64) / (sample_size as f64);
        Self {
            iter_time_ns,
            sample_size,
        }
    }
}
