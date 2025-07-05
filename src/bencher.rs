use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use crate::label::Label;

#[derive(Default)]
pub struct Bencher {
    config: Config,
    results: Vec<BenchResult>,
}

impl Bencher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bench_function<R, F>(&mut self, label: impl Into<Label>, mut func: F)
    where
        F: FnMut() -> R,
    {
        let warmup_start = Instant::now();
        let mut warmup_iters = 0;
        while warmup_start.elapsed() < self.config.warmup {
            func();
            warmup_iters += 1;
        }

        let time_single_run = warmup_start.elapsed().div_f64(warmup_iters as f64);

        let runs_needed = self
            .config
            .sample_time
            .div_duration_f32(time_single_run)
            .ceil() as usize;

        let now = Instant::now();
        for _ in 0..runs_needed {
            func();
        }
        let time = now.elapsed().div_f64(runs_needed as f64);
        let res = BenchResult {
            label: label.into(),
            time,
        };
        eprintln!("{res}");

        self.results.push(res);
    }
}

struct Config {
    warmup: Duration,
    sample_time: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            warmup: Duration::from_millis(500),
            sample_time: Duration::from_secs(2),
        }
    }
}

#[derive(Debug)]
struct BenchResult {
    label: Label,
    time: Duration,
}

impl Display for BenchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.label, self.time)
    }
}
